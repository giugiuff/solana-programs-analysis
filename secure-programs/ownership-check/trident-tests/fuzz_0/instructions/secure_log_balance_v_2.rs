// Sets up the secure_log_balance_v_2 instruction for the Ownership Check fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::account::AccountSharedData;
use solana_sdk::rent::Rent;
use std::str::FromStr;
use spl_token::solana_program::program_option::COption;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplMint};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("4u6h9QAMT8TuXauVZa9ieeQext18EtjecnX95xxw4xaa")]
#[discriminator([54u8, 104u8, 158u8, 231u8, 47u8, 12u8, 194u8, 90u8])]
pub struct SecureLogBalanceV2Instruction {
    pub accounts: SecureLogBalanceV2InstructionAccounts,
    pub data: SecureLogBalanceV2InstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SecureLogBalanceV2InstructionData)]
#[storage(FuzzAccounts)]
pub struct SecureLogBalanceV2InstructionAccounts {
    pub mint: TridentAccount,

    pub token_account: TridentAccount,

    #[account(signer)]
    pub token_account_owner: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SecureLogBalanceV2InstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for SecureLogBalanceV2Instruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const ASSOCIATED_TOKEN_PROGRAM_ID: &str =
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

        let scenario = &fuzz_accounts.ownership;

        let legit_owner = fuzz_accounts.token_account_owner.get_or_create(
            scenario.legit_owner_id,
            trident,
            None,
            None,
        );
        let attacker_owner = fuzz_accounts.token_account_owner.get_or_create(
            scenario.attacker_owner_id,
            trident,
            None,
            None,
        );

        let declared_owner = if scenario.use_correct_owner {
            legit_owner
        } else {
            attacker_owner
        };

        self.accounts
            .token_account_owner
            .set_address(declared_owner);
        self.accounts.token_account_owner.set_is_signer();
        self.accounts.token_account_owner.account_id = if scenario.use_correct_owner {
            scenario.legit_owner_id
        } else {
            scenario.attacker_owner_id
        };

        let rent = Rent::default();
        let token_program = Pubkey::new_from_array(spl_token::id().to_bytes());

        let mint_pubkey =
            fuzz_accounts
                .mint
                .get_or_create(scenario.mint_account_id, trident, None, None);
        let mut mint_account = AccountSharedData::new(
            rent.minimum_balance(SplMint::LEN),
            SplMint::LEN,
            &token_program,
        );

        let mint_authority = trident.gen_pubkey();
        let mint_authority_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(mint_authority.to_bytes());
        let decimals = trident.gen_range(0..=9);

        let mut mint_bytes = vec![0u8; SplMint::LEN];
        SplMint::pack(
            SplMint {
                is_initialized: true,
                mint_authority: COption::Some(mint_authority_spl),
                freeze_authority: COption::None,
                decimals,
                ..SplMint::default()
            },
            &mut mint_bytes,
        )
        .expect("pack mint");
        mint_account.set_data_from_slice(&mint_bytes);
        trident
            .get_client()
            .set_account_custom(&mint_pubkey, &mint_account);

        self.accounts.mint.set_address(mint_pubkey);
        self.accounts.mint.set_is_writable();
        self.accounts.mint.account_id = scenario.mint_account_id;

        let actual_owner = if scenario.use_correct_owner {
            declared_owner
        } else {
            legit_owner
        };

        let owner_bytes = actual_owner.to_bytes();
        let token_program_bytes = token_program.to_bytes();
        let mint_bytes = mint_pubkey.to_bytes();

        let associated_token_program =
            Pubkey::from_str(ASSOCIATED_TOKEN_PROGRAM_ID).expect("associated token program id");

        let seed_slices: [&[u8]; 3] = [
            owner_bytes.as_ref(),
            token_program_bytes.as_ref(),
            mint_bytes.as_ref(),
        ];

        let (token_account_pubkey, _) =
            Pubkey::find_program_address(&seed_slices, &associated_token_program);
        let mut token_account = AccountSharedData::new(
            rent.minimum_balance(SplTokenAccount::LEN),
            SplTokenAccount::LEN,
            &token_program,
        );

        let mint_pubkey_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(mint_bytes);
        let actual_owner_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(owner_bytes);

        let mut token_bytes = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(
            SplTokenAccount {
                mint: mint_pubkey_spl,
                owner: actual_owner_spl,
                amount: trident.gen_range(0..1_000_000_000u64),
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            &mut token_bytes,
        )
        .expect("pack token account");
        token_account.set_data_from_slice(&token_bytes);
        trident
            .get_client()
            .set_account_custom(&token_account_pubkey, &token_account);

        self.accounts
            .token_account
            .set_address(token_account_pubkey);
        self.accounts.token_account.set_is_writable();
        self.accounts.token_account.account_id = scenario.token_account_id;
    }
}
