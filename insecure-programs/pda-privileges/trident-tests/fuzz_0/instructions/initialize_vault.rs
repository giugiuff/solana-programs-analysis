use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("4hKSDzDDxaHcdCDvULi2i5hHUsQ5oiS9NFx3uL1qtnoc")]
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
        let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
            .expect("valid token program id");
        let ata_program = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
            .expect("valid associated token program id");

        // Choose a persistent victim account that acts as the vault creator.
        let victim_pubkey = fuzz_accounts
            .vault_creator
            .get_or_create(0, trident, None, None);
        self.accounts.vault_creator.set_address(victim_pubkey);
        self.accounts.vault_creator.set_is_signer();
        self.accounts.vault_creator.set_is_writable();

        // Reuse a single mint shared by the tests.
        let mint_pubkey = fuzz_accounts.mint.get_or_create_mint_account(
            self.accounts.mint.account_id,
            trident,
            None,
            0,
            &token_program,
            None,
        );
        self.accounts.mint.set_address(mint_pubkey);

        // Derive metadata PDA that will be created by the instruction.
        let metadata_seeds: &[&[u8]] = &[b"metadata_account", victim_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(metadata_seeds, &program_id);
        self.accounts.metadata_account.set_address(metadata_pubkey);
        self.accounts.metadata_account.set_is_writable();

        // Derive the vault ATA (created by the associated token program).
        let vault_seeds: &[&[u8]] = &[
            metadata_pubkey.as_ref(),
            token_program.as_ref(),
            mint_pubkey.as_ref(),
        ];
        let (vault_pubkey, _) = Pubkey::find_program_address(vault_seeds, &ata_program);
        self.accounts.vault.set_address(vault_pubkey);
        self.accounts.vault.set_is_writable();

        // Static program accounts.
        self.accounts.token_program.set_address(token_program);
        self.accounts
            .associated_token_program
            .set_address(ata_program);
        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
    }
}

fn program_pubkey() -> Pubkey {
    Pubkey::from_str("4hKSDzDDxaHcdCDvULi2i5hHUsQ5oiS9NFx3uL1qtnoc").expect("valid program id")
}
