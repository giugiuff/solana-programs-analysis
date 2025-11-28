// Sets up the initialize_secure instruction for the Initialization Frontrunning fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::solana_sdk::bpf_loader_upgradeable;
use trident_fuzz::fuzzing::*;

const PROGRAM_ID: Pubkey = pubkey!("8GN5wnP7R8eUhvHYW8hf9hWjyc8PoU4mEWDCSHsyESQ3");

#[derive(TridentInstruction, Default)]
#[program_id("8GN5wnP7R8eUhvHYW8hf9hWjyc8PoU4mEWDCSHsyESQ3")]
#[discriminator([22u8, 242u8, 50u8, 101u8, 87u8, 199u8, 204u8, 53u8])]
pub struct InitializeSecureInstruction {
    pub accounts: InitializeSecureInstructionAccounts,
    pub data: InitializeSecureInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeSecureInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeSecureInstructionAccounts {
    #[account(mut, signer)]
    pub signer: TridentAccount,

    #[account(mut)]
    pub global_config: TridentAccount,

    pub program_data: TridentAccount,

    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeSecureInstructionData {
    pub additional_data: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeSecureInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, _trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        self.data.additional_data = fuzz_accounts.initialization.additional_data;
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let scenario = &fuzz_accounts.initialization;

        let legit_signer =
            fuzz_accounts
                .signer
                .get_or_create(scenario.legit_signer_id, trident, None, None);

        let attacker_signer =
            fuzz_accounts
                .signer
                .get_or_create(scenario.attacker_signer_id, trident, None, None);

        let signer_pubkey = if scenario.use_correct_authority {
            legit_signer
        } else {
            attacker_signer
        };

        self.accounts.signer.set_address(signer_pubkey);
        self.accounts.signer.set_is_signer();
        self.accounts.signer.account_id = if scenario.use_correct_authority {
            scenario.legit_signer_id
        } else {
            scenario.attacker_signer_id
        };

        let global_config = fuzz_accounts.global_config.get_or_create(
            scenario.global_config_id,
            trident,
            Some(PdaSeeds::new(&[b"config"], PROGRAM_ID)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );
        self.accounts.global_config.set_address(global_config);
        self.accounts.global_config.set_is_writable();
        self.accounts.global_config.account_id = scenario.global_config_id;

        let program_data = fuzz_accounts.program_data.get_or_create(
            scenario.program_data_id,
            trident,
            Some(PdaSeeds::new(
                &[PROGRAM_ID.as_ref()],
                bpf_loader_upgradeable::ID,
            )),
            None,
        );
        self.accounts.program_data.set_address(program_data);
        self.accounts.program_data.account_id = scenario.program_data_id;

        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
    }
}


