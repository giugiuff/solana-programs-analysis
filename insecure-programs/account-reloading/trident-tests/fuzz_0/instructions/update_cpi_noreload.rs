use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(TridentInstruction, Default)]
#[program_id("3Rp7wE8v6S5Sd6mqikGQWo1tQ2jC8DfSPm1mf31zUGgH")]
#[discriminator([255u8, 234u8, 174u8, 213u8, 130u8, 238u8, 85u8, 57u8])]
pub struct UpdateCpiNoreloadInstruction {
    pub accounts: UpdateCpiNoreloadInstructionAccounts,
    pub data: UpdateCpiNoreloadInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(UpdateCpiNoreloadInstructionData)]
#[storage(FuzzAccounts)]
pub struct UpdateCpiNoreloadInstructionAccounts {
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
        //seeds = [b"metadata",authority.as_ref()],
    )]
    pub metadata: TridentAccount,

    #[account(address = "94WcpeofNg8iKm4u8Veh5EYYhpTzDfaWGe7WknzKuNeG")]
    pub update_account: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct UpdateCpiNoreloadInstructionData {
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
impl InstructionHooks for UpdateCpiNoreloadInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, trident: &mut Trident, _fa: &mut Self::IxAccounts) {
        self.data.new_input = trident.gen_range(0u8..=u8::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fa: &mut FuzzAccounts) {
        // authority stabile (id 0)
        let auth = fa.authority.get_or_create(0, trident, None, None);
        self.accounts.authority.set_account_meta(auth, true, false);

        // PDA `metadata` del programma update_account: seeds = [b"metadata", authority]
        let ua_pid = Pubkey::from_str("94WcpeofNg8iKm4u8Veh5EYYhpTzDfaWGe7WknzKuNeG").unwrap();
        let (pda, _bump) = Pubkey::find_program_address(&[b"metadata", auth.as_ref()], &ua_pid);
        self.accounts.metadata.set_account_meta(pda, false, true);

        // Evitiamo che il nuovo input coincida con quello attuale, così da evidenziare la mancata reload.
        let current_account = trident.get_client().get_account(&pda);
        if current_account.data().len() > 8 {
            if let Ok(on_chain_meta) = Metadata::try_from_slice(&current_account.data()[8..]) {
                if on_chain_meta.input == self.data.new_input {
                    self.data.new_input = on_chain_meta.input.wrapping_add(1);
                }
            }
        }

        // il program account è già fissato dall'attributo address
    }
}


