use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("FufZySVu1t5niHkT1Botiyssi9cgUcQtBXJQtNsiRbwM")]
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
    #[account(
        mut, 
        signer,
        storage::name = vault_authority,
        storage::account_id = (0..1),
    )]
    pub vault_authority: TridentAccount,

    #[account(
        mut,
        signer,
        storage::name = vault,
        storage::account_id = (0..1),
    )]
    pub vault: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeVaultInstructionData {
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
    //qui specificare quali dati random per l'initialize instruction
    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
    self.data.vault_data = trident.gen_range(0..u8::MAX);
    }

}