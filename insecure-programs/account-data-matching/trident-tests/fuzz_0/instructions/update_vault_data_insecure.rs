use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("FufZySVu1t5niHkT1Botiyssi9cgUcQtBXJQtNsiRbwM")]
#[discriminator([192u8, 128u8, 121u8, 253u8, 172u8, 129u8, 203u8, 211u8])]
pub struct UpdateVaultDataInsecureInstruction {
    pub accounts: UpdateVaultDataInsecureInstructionAccounts,
    pub data: UpdateVaultDataInsecureInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(UpdateVaultDataInsecureInstructionData)]
#[storage(FuzzAccounts)]
pub struct UpdateVaultDataInsecureInstructionAccounts {
    #[account(
        mut,
        signer,
        storage::name = vault_authority,
        storage::account_id = (0..2)
    )]
    pub vault_authority: TridentAccount,

    #[account(
        mut,
        //signer,
        storage::name = vault, 
        storage::account_id = (0..1),
    )]
    pub vault: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct UpdateVaultDataInsecureInstructionData {
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
impl InstructionHooks for UpdateVaultDataInsecureInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
    self.data.new_data = trident.gen_range(0..u8::MAX);
    }
   
}
