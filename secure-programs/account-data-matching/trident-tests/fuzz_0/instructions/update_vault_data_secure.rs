// Sets up the update_vault_data_secure instruction for the Account Data Matching fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

// Deriving TridentInstruction again exposes this update instruction to Trident.
#[derive(TridentInstruction, Default)]
#[program_id("Mq8ZJubhF7DShnSE3vnyVpqj2SCA9kinJk75VwvNof1")]
#[discriminator([58u8, 138u8, 167u8, 72u8, 156u8, 83u8, 82u8, 28u8])]
pub struct UpdateVaultDataSecureInstruction {
    pub accounts: UpdateVaultDataSecureInstructionAccounts,
    pub data: UpdateVaultDataSecureInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(UpdateVaultDataSecureInstructionData)]
#[storage(FuzzAccounts)]
pub struct UpdateVaultDataSecureInstructionAccounts {
    #[account(
        mut,
        signer,
        storage::name = vault_authority,
        storage::account_id = (0..2),
    )]
    pub vault_authority: TridentAccount,

    // Writable vault account that holds the byte under test; fuzzing inspects it after execution.
    #[account(
        mut,
        storage::name = vault, 
        storage::account_id = (0..1),
    )]
    pub vault: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct UpdateVaultDataSecureInstructionData {
    // Byte the updater wants to write; invariants later confirm only the owner can change it.
    pub new_data: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for UpdateVaultDataSecureInstruction {
    type IxAccounts = FuzzAccounts;
    // Randomize the requested value so the invariant sees both valid and invalid transitions.
    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        self.data.new_data = trident.gen_range(0..u8::MAX);
    }
}
