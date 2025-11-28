// Registers the Initialization Frontrunning fuzz suite with Trident and drives the generated transactions during fuzzing.

use fuzz_accounts::*;
use trident_fuzz::fuzzing::solana_sdk::account::AccountSharedData;
use trident_fuzz::fuzzing::solana_sdk::pubkey::Pubkey;
use trident_fuzz::fuzzing::solana_sdk::{bpf_loader_upgradeable, system_program};
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use types::{InitializationScenario, ATTACKER_SIGNER_ID, LEGIT_SIGNER_ID};

const PROGRAM_ID: Pubkey = pubkey!("8GN5wnP7R8eUhvHYW8hf9hWjyc8PoU4mEWDCSHsyESQ3");

#[derive(FuzzTestMethods)]
struct FuzzTest {
    trident: Trident,
    fuzz_accounts: FuzzAccounts,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) {}

    #[flow]
    fn flow_initialize_secure(&mut self) {
        self.fuzz_accounts.initialization = generate_initialization_scenario(&mut self.trident);
        prepare_initialization_accounts(&mut self.trident, &mut self.fuzz_accounts);

        let mut tx = InitializeSecureTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("initialize_secure"));
    }

    #[end]
    fn end(&mut self) {}
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}

fn generate_initialization_scenario(trident: &mut Trident) -> InitializationScenario {
    let use_correct_authority = trident.gen_range(0..100_u8) < 50;
    let additional_data = trident.gen_range(0..=u8::MAX);

    InitializationScenario {
        use_correct_authority,
        additional_data,
        ..InitializationScenario::default()
    }
}

fn prepare_initialization_accounts(trident: &mut Trident, fuzz_accounts: &mut FuzzAccounts) {
    let scenario = &fuzz_accounts.initialization;

    let legit_signer = fuzz_accounts
        .signer
        .get_or_create(LEGIT_SIGNER_ID, trident, None, None);
    let attacker_signer =
        fuzz_accounts
            .signer
            .get_or_create(ATTACKER_SIGNER_ID, trident, None, None);

    trident.airdrop(&legit_signer, 5 * LAMPORTS_PER_SOL);
    trident.airdrop(&attacker_signer, 5 * LAMPORTS_PER_SOL);

    let global_config = fuzz_accounts.global_config.get_or_create(
        scenario.global_config_id,
        trident,
        Some(PdaSeeds::new(&[b"config"], PROGRAM_ID)),
        Some(AccountMetadata::new(0, 0, system_program::ID)),
    );
    let global_config_account = AccountSharedData::new(0, 0, &system_program::ID);
    trident
        .get_client()
        .set_account_custom(&global_config, &global_config_account);

    let program_data = fuzz_accounts.program_data.get_or_create(
        scenario.program_data_id,
        trident,
        Some(PdaSeeds::new(
            &[PROGRAM_ID.as_ref()],
            bpf_loader_upgradeable::ID,
        )),
        None,
    );
    let mut program_data_account = trident.get_client().get_account(&program_data);
    let mut data = program_data_account.data().to_vec();
    let metadata_len =
        bpf_loader_upgradeable::UpgradeableLoaderState::size_of_programdata_metadata();
    assert!(data.len() >= metadata_len);

    let authority_offset = 4 + 8;
    data[authority_offset] = 1;
    let authority_bytes = legit_signer.to_bytes();
    data[authority_offset + 1..authority_offset + 1 + 32].copy_from_slice(authority_bytes.as_ref());

    program_data_account.set_data_from_slice(&data);
    trident
        .get_client()
        .set_account_custom(&program_data, &program_data_account);
}
