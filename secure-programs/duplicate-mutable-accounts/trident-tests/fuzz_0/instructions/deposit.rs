// Sets up the deposit instruction for the Duplicate Mutable Accounts fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("3CGZ6JWo2QENGahAopnv9Z5mwCHH6WgKKbLJk9y7zLu8")]
#[discriminator([242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8])]
pub struct DepositInstruction {
    pub accounts: DepositInstructionAccounts,
    pub data: DepositInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(DepositInstructionData)]
#[storage(FuzzAccounts)]
pub struct DepositInstructionAccounts {
    #[account(signer)]
    pub owner: TridentAccount,

    #[account(mut)]
    pub vault: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct DepositInstructionData {
    pub amount: u64,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for DepositInstruction {
    type IxAccounts = FuzzAccounts;
}
