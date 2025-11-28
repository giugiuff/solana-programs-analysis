// Sets up the update_cpi_reload instruction for the Account Reloading fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("8DjgPHfzsoDeQxiQHsv3ggKhNUzevHBEarftcgwntyaB")]
#[discriminator([211u8, 6u8, 172u8, 202u8, 27u8, 68u8, 181u8, 244u8])]
pub struct UpdateCpiReloadInstruction {
    pub accounts: UpdateCpiReloadInstructionAccounts,
    pub data: UpdateCpiReloadInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(UpdateCpiReloadInstructionData)]
#[storage(FuzzAccounts)]
pub struct UpdateCpiReloadInstructionAccounts {
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
    )]
    pub metadata: TridentAccount,

    #[account(address = "J7sgpJXG4fDUa3vbdTwjtTWkgnxxcHbg57hLnyQ6vRnH")]
    pub update_account: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct UpdateCpiReloadInstructionData {
    pub new_input: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for UpdateCpiReloadInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fa: &mut Self::IxAccounts) {
        self.data.new_input = trident.gen_range(0u8..=u8::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fa: &mut FuzzAccounts) {
        let auth = fa.authority.get_or_create(0, trident, None, None);
        self.accounts.authority.set_account_meta(auth, true, false);

        let ua_pid = Pubkey::from_str("J7sgpJXG4fDUa3vbdTwjtTWkgnxxcHbg57hLnyQ6vRnH").unwrap();
        let (pda, _bump) = Pubkey::find_program_address(&[b"metadata", auth.as_ref()], &ua_pid);
        self.accounts.metadata.set_account_meta(pda, false, true);

        let current_account = trident.get_client().get_account(&pda);
        if current_account.data().len() > 8 {
            if let Ok(on_chain_meta) = Metadata::try_from_slice(&current_account.data()[8..]) {
                if on_chain_meta.input == self.data.new_input {
                    self.data.new_input = on_chain_meta.input.wrapping_add(1);
                }
            }
        }
    }
}





