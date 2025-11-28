

use fuzz_accounts::*;
use trident_fuzz::fuzzing::AccountSharedData;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use types::{ATTACKER_ID, INITIAL_LAMPORTS, LEGIT_CREATOR_ID, PROGRAM_ID};

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
    fn start(&mut self) {
        // Prepariamo i signer deterministici (legittimo + attaccante) prima dell'esecuzione dei flow.
        self.prepare_signers();
    }

    #[flow]
    fn secure_initialize_flow(&mut self) {
        // Eseguiamo sia il creatore legittimo sia l'attaccante in sequenza.
        // Variante sicura: la prima call riesce, la seconda (attaccante) deve andare in panic.
        self.reset_metadata_account();
        self.run_secure(false);
        self.run_secure(true);
    }

    #[end]
    fn end(&mut self) {}
}

impl FuzzTest {
    fn prepare_signers(&mut self) {
        // Deriviamo chiavi deterministiche per le identità del creatore e dell'attaccante.
        let legit = self.fuzz_accounts.creator.get_or_create(
            LEGIT_CREATOR_ID,
            &mut self.trident,
            None,
            None,
        );
        let attacker =
            self.fuzz_accounts
                .attacker
                .get_or_create(ATTACKER_ID, &mut self.trident, None, None);

        // Finanzia entrambi così le transazioni successive sono rent-exempt.
        self.trident.airdrop(&legit, INITIAL_LAMPORTS);
        self.trident.airdrop(&attacker, INITIAL_LAMPORTS);
    }

    fn reset_metadata_account(&mut self) {
        // Reset del metadata PDA in un account di sistema vuoto per permettere la nuova init.
        let (metadata_pubkey, _) = Pubkey::find_program_address(&[b"metadata"], &PROGRAM_ID);

        let account = AccountSharedData::new(0, 0, &solana_sdk::system_program::ID);
        self.trident
            .get_client()
            .set_account_custom(&metadata_pubkey, &account);
    }

    fn run_secure(&mut self, as_attacker: bool) {
        // La call legittima deve riuscire, quella dell'attaccante deve andare in panic (gestito dall'hook).
        self.fuzz_accounts.call_as_attacker = as_attacker;
        let mut tx = SecureInitializeTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("secure_initialize"));
        self.fuzz_accounts.call_as_attacker = false;
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
