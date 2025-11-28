use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("6HHWk9zBVkq8XNTTpE9tidRjjfTSEkAQeTqi3hnB6xsW")]
#[discriminator([95u8, 197u8, 159u8, 142u8, 189u8, 29u8, 159u8, 21u8])]
pub struct VerifyPinInstruction {
    pub accounts: VerifyPinInstructionAccounts,
    pub data: VerifyPinInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(VerifyPinInstructionData)]
#[storage(FuzzAccounts)]
pub struct VerifyPinInstructionAccounts {
    pub author: TridentAccount,

    pub secret_information: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct VerifyPinInstructionData {
    pub _pin1: u8,

    pub _pin2: u8,

    pub _pin3: u8,

    pub _pin4: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for VerifyPinInstruction {
    type IxAccounts = FuzzAccounts;
}