// Sets up the initialize_vault instruction for the PDA Privileges fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use spl_token::solana_program::program_option::COption;
use spl_token::solana_program::program_pack::Pack;
use spl_token::solana_program::pubkey::Pubkey as SplTokenPubkey;
use spl_token::state::Mint as SplMint;
use std::str::FromStr;
use trident_fuzz::fuzzing::solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_fuzz::fuzzing::AccountMetadata;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("J6SWoen6bHQQEnPpU8TUiU46ABpthjcvrYeiXCcUXa17")]
#[discriminator([48u8, 191u8, 163u8, 44u8, 71u8, 129u8, 63u8, 164u8])]
pub struct InitializeVaultInstruction {
    pub accounts: InitializeVaultInstructionAccounts,
    pub data: InitializeVaultInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeVaultInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeVaultInstructionAccounts {
    #[account(mut, signer)]
    pub vault_creator: TridentAccount,

    #[account(mut)]
    pub vault: TridentAccount,

    #[account(mut)]
    pub metadata_account: TridentAccount,

    pub mint: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,

    #[account(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")]
    pub associated_token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeVaultInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeVaultInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let program_id = program_pubkey();
        let token_program = token_program_pubkey();
        let ata_program = associated_token_program_pubkey();

        let vault_creator =
            fuzz_accounts
                .vault_creator
                .get_or_create(LEGIT_CREATOR_ID, trident, None, None);
        let vault_creator_pubkey = Pubkey::new_from_array(vault_creator.to_bytes());
        trident.airdrop(&vault_creator, 5 * LAMPORTS_PER_SOL);
        self.accounts.vault_creator.set_address(vault_creator);
        self.accounts.vault_creator.set_is_signer();
        self.accounts.vault_creator.set_is_writable();
        self.accounts.vault_creator.account_id = LEGIT_CREATOR_ID;

        let mint_pubkey = fuzz_accounts.mint.get_or_create(
            MINT_ACCOUNT_ID,
            trident,
            None,
            Some(AccountMetadata::new(
                Rent::default().minimum_balance(SplMint::LEN),
                SplMint::LEN,
                token_program,
            )),
        );
        self.accounts.mint.set_address(mint_pubkey);
        self.accounts.mint.set_is_writable();
        self.accounts.mint.account_id = MINT_ACCOUNT_ID;

        let mut mint_account = trident.get_client().get_account(&mint_pubkey);
        let authority_key = SplTokenPubkey::new_from_array(vault_creator_pubkey.to_bytes());
        let mut mint_bytes = vec![0u8; SplMint::LEN];
        SplMint::pack(
            SplMint {
                mint_authority: COption::Some(authority_key),
                supply: 0,
                decimals: 0,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            &mut mint_bytes,
        )
        .expect("pack mint state");
        mint_account.set_data_from_slice(&mint_bytes);
        trident
            .get_client()
            .set_account_custom(&mint_pubkey, &mint_account);

        let metadata_seeds: &[&[u8]] = &[b"metadata_account", vault_creator_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(metadata_seeds, &program_id);
        self.accounts.metadata_account.set_address(metadata_pubkey);
        self.accounts.metadata_account.set_is_writable();
        self.accounts.metadata_account.account_id = METADATA_ACCOUNT_ID;

        let vault_seeds: &[&[u8]] = &[
            metadata_pubkey.as_ref(),
            token_program.as_ref(),
            mint_pubkey.as_ref(),
        ];
        let (vault_pubkey, _) = Pubkey::find_program_address(vault_seeds, &ata_program);
        self.accounts.vault.set_address(vault_pubkey);
        self.accounts.vault.set_is_writable();
        self.accounts.vault.account_id = VAULT_ACCOUNT_ID;

        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
        self.accounts.token_program.set_address(token_program);
        self.accounts
            .associated_token_program
            .set_address(ata_program);
    }
}

fn program_pubkey() -> Pubkey {
    Pubkey::from_str("J6SWoen6bHQQEnPpU8TUiU46ABpthjcvrYeiXCcUXa17").expect("valid program id")
}

fn token_program_pubkey() -> Pubkey {
    Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").expect("valid token program id")
}

fn associated_token_program_pubkey() -> Pubkey {
    Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
        .expect("valid associated token program id")
}
