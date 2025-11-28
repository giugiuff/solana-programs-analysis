

use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Runtime Trident che alimenta il fuzz.
    trident: Trident,
    /// Archivio degli account usati durante i test.
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
    fn initialize_metadata(&mut self) {
        // Bootstrap iniziale: crea il metadata per poter testare le chiusure/revival.
        let mut tx =
            InitializeMetadataTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("initialize_metadata"));
    }


    #[flow]
    fn verify_pin_revival(&mut self) {
        // Chiude il metadata e poi tenta la verifica del PIN per riprodurre l'attacco di revival.
        let mut close_tx =
            CloseMetadataTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut close_tx, Some("close_metadata"));

        let mut verify_tx =
            VerifyPinTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut verify_tx, Some("verify_pin"));
    }

    #[end]
    fn end(&mut self) {
        
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
