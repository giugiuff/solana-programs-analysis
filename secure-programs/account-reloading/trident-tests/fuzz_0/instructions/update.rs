// Sets up the update instruction for the Account Reloading fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
use trident_fuzz::traits::AccountsMethods;

#[derive(TridentInstruction, Default)]
#[program_id("J7sgpJXG4fDUa3vbdTwjtTWkgnxxcHbg57hLnyQ6vRnH")]
#[discriminator([219u8, 200u8, 88u8, 176u8, 158u8, 63u8, 253u8, 127u8])]
pub struct UpdateInstruction {
    pub accounts: UpdateInstructionAccounts,
    pub data: UpdateInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(UpdateInstructionData)]
#[storage(FuzzAccounts)]
pub struct UpdateInstructionAccounts {
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
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct UpdateInstructionData {
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
impl InstructionHooks for UpdateInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        // copre 0..=255
        self.data.input = trident.gen_range(0u8..=u8::MAX);
    }

}



