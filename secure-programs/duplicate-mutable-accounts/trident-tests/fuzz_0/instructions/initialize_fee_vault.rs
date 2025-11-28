// Sets up the initialize_fee_vault instruction for the Duplicate Mutable Accounts fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("3CGZ6JWo2QENGahAopnv9Z5mwCHH6WgKKbLJk9y7zLu8")]
#[discriminator([185u8, 140u8, 228u8, 234u8, 79u8, 203u8, 252u8, 50u8])]
pub struct InitializeFeeVaultInstruction {
    pub accounts: InitializeFeeVaultInstructionAccounts,
    pub data: InitializeFeeVaultInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeFeeVaultInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeFeeVaultInstructionAccounts {
    #[account(mut, signer)]
    pub authority: TridentAccount,

    #[account(mut)]
    pub vault: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeFeeVaultInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeFeeVaultInstruction {
    type IxAccounts = FuzzAccounts;
}
