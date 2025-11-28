// Sets up the initialize_vault instruction for the Account Data Matching fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

// Deriving TridentInstruction wires this struct into the fuzz harness glue code.
#[derive(TridentInstruction, Default)]
#[program_id("Mq8ZJubhF7DShnSE3vnyVpqj2SCA9kinJk75VwvNof1")]
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
    // Vault authority signer seeded deterministically so ownership enforcement can be fuzzed.
    #[account(
        mut, 
        signer,
        storage::name = vault_authority,
        storage::account_id = (0..1),
    )]
    pub vault_authority: TridentAccount,

    // PDA storing the matched byte; tagging as writable lets us snapshot its data.
    #[account(
        mut,
        signer,
        storage::name = vault,
        storage::account_id = (0..1),
    )]
    pub vault: TridentAccount,

    // Keep the system program fixed to avoid randomization of core programs.
    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeVaultInstructionData {
    // Initial byte committed during initialization; later flows check who can mutate it.
    pub vault_data: u8,
}

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
    // Randomize the stored byte each iteration so unauthorized writes are easier to detect.
    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
    self.data.vault_data = trident.gen_range(0..u8::MAX);
    }

}
