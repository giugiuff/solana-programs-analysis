use fuzz_accounts::*; // esporta lo storage condiviso fra gli hook
use trident_fuzz::fuzzing::*; // macro e API per definire il flusso di fuzzing
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Runner Trident che orchestrerà tutte le transazioni generate.
    trident: Trident,
    /// Stato condiviso che memorizza PDA e flag di inizializzazione.
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
        // Inizializza l'account utente (solo una volta per la campagna di fuzzing).
        if !self.fuzz_accounts.user_initialized {
            let mut init_user =
                InitializeUserTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
            self.trident
                .execute_transaction(&mut init_user, Some("initialize_user"));
            self.fuzz_accounts.user_initialized = true;
        }

        // In modo analogo crea l'account dei metadati se non esiste già.
        if !self.fuzz_accounts.metadata_initialized {
            let mut init_metadata = InitializeUserMetadataTransaction::build(
                &mut self.trident,
                &mut self.fuzz_accounts,
            );
            self.trident
                .execute_transaction(&mut init_metadata, Some("initialize_user_metadata"));
            self.fuzz_accounts.metadata_initialized = true;
        }
    }

    #[flow]
    fn flow_insecure_user_read(&mut self) {
        // Costruisce ed esegue l'istruzione vulnerabile di lettura con type cosplay.
        let mut tx = InsecureUserReadTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("insecure_user_read"));
    }

    #[end]
    fn end(&mut self) {}
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
