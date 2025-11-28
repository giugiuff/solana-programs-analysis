// Sets up the initialize_vault instruction for the Duplicate Mutable Accounts fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("3CGZ6JWo2QENGahAopnv9Z5mwCHH6WgKKbLJk9y7zLu8")]
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
    pub creator: TridentAccount,

    #[account(mut)]
    pub vault: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
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
}
