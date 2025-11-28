// Sets up the initialize instruction for the Account Reloading fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("J7sgpJXG4fDUa3vbdTwjtTWkgnxxcHbg57hLnyQ6vRnH")]
#[discriminator([175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeInstructionAccounts {
    #[account(
        mut,
        signer,
        storage::name = authority,
        storage::account_id = (0..1),
    )]
    pub authority: TridentAccount,

    #[account(
        mut,
        storage::name = metadata,
        storage::account_id = (0..1),
        seeds = [b"metadata",authority.as_ref()],
            )]
    pub metadata: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeInstructionData {
    pub input: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        self.data.input = trident.gen_range(0..u8::MAX);
    }
}



