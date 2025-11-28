use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::account::AccountSharedData;
use solana_sdk::rent::Rent;
use spl_token::solana_program::program_option::COption;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplMint};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("GhD5bDw7vBR7mo9ET56VNH85ThYqXTuWdebpGTpHvaoj")]
#[discriminator([137u8, 102u8, 172u8, 0u8, 128u8, 60u8, 94u8, 177u8])]
pub struct InsecureLogBalanceV1Instruction {
    pub accounts: InsecureLogBalanceV1InstructionAccounts,
    pub data: InsecureLogBalanceV1InstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InsecureLogBalanceV1InstructionData)]
#[storage(FuzzAccounts)]
pub struct InsecureLogBalanceV1InstructionAccounts {
    #[account(
        storage::name = mint,
        storage::account_id = (0..5),
    )]
    pub mint: TridentAccount,

    #[account(
        storage::name = token_account,
        storage::account_id = (0..5),
    )]
    pub token_account: TridentAccount,

    #[account(
        signer,
        storage::name = token_account_owner,
        storage::account_id = (0..5),
    )]
    pub token_account_owner: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InsecureLogBalanceV1InstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
// Prepara gli account della CPI forzando talvolta un owner scorretto per evidenziare il bug V1.
impl InstructionHooks for InsecureLogBalanceV1Instruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Definisce un nuovo mint SPL casuale riutilizzando lo slot di storage assegnato.
        let mint_account_id = self.accounts.mint.account_id;
        let decimals = trident.gen_range(0..=9);
        let mint_authority = trident.gen_pubkey();

        // Riutilizziamo lo storage per avere un mint SPL coerente fra le iterazioni.
        let mint_pubkey = fuzz_accounts
            .mint
            .get_or_create(mint_account_id, trident, None, None);

        let rent = Rent::default();
        let token_program = Pubkey::new_from_array(spl_token::id().to_bytes());
        let mut mint_account = AccountSharedData::new(
            rent.minimum_balance(SplMint::LEN),
            SplMint::LEN,
            &token_program,
        );

        let mint_authority_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(mint_authority.to_bytes());

        let mint_state = SplMint {
            is_initialized: true,
            mint_authority: COption::Some(mint_authority_spl),
            freeze_authority: COption::None,
            decimals,
            ..SplMint::default()
        };
        let mut mint_data = vec![0u8; SplMint::LEN];
        SplMint::pack(mint_state, &mut mint_data).unwrap();
        mint_account.set_data_from_slice(&mint_data);
        trident
            .get_client()
            .set_account_custom(&mint_pubkey, &mint_account);

        self.accounts.mint.set_address(mint_pubkey);

        let owner_storage_id = self.accounts.token_account_owner.account_id;
        let provided_owner_pubkey =
            fuzz_accounts
                .token_account_owner
                .get_or_create(owner_storage_id, trident, None, None);
        self.accounts
            .token_account_owner
            .set_address(provided_owner_pubkey);

        let mut actual_owner = provided_owner_pubkey;
        if trident.gen_range(0..100) < 50 {
            // Con metà probabilità imponiamo un owner diverso per scatenare la vulnerabilità.
            loop {
                let candidate = trident.gen_pubkey();
                if candidate != provided_owner_pubkey {
                    actual_owner = candidate;
                    break;
                }
            }
        }

        // Crea o recupera un token account associandolo eventualmente a un proprietario non autorizzato.
        let token_account_id = self.accounts.token_account.account_id;
        let token_balance = trident.gen_range(0..1_000_000_000u64);

        let token_account_pubkey =
            fuzz_accounts
                .token_account
                .get_or_create(token_account_id, trident, None, None);

        let mut token_account_data = AccountSharedData::new(
            rent.minimum_balance(SplTokenAccount::LEN),
            SplTokenAccount::LEN,
            &token_program,
        );

        let mint_pubkey_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(mint_pubkey.to_bytes());
        let actual_owner_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(actual_owner.to_bytes());

        // Ricostruiamo a mano l'account token per bypassare il controllo ownership di Anchor.
        let spl_account = SplTokenAccount {
            mint: mint_pubkey_spl,
            owner: actual_owner_spl,
            amount: token_balance,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        };

        let mut token_bytes = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(spl_account, &mut token_bytes).unwrap();
        token_account_data.set_data_from_slice(&token_bytes);
        trident
            .get_client()
            .set_account_custom(&token_account_pubkey, &token_account_data);

        self.accounts
            .token_account
            .set_address(token_account_pubkey);
    }
}
