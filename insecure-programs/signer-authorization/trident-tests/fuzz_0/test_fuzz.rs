use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

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
        if !self.fuzz_accounts.escrow_initialized {
            // Inizializza l'escrow bersaglio solo la prima volta.
            let mut tx = InitializeTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
            self.trident
                .execute_transaction(&mut tx, Some("initialize"));
            self.fuzz_accounts.escrow_initialized = true;
        }
    }

    #[flow]
    fn flow_insecure_authorization(&mut self) {
        // Esegue l'istruzione insicura con parametri randomizzati dal fuzzer.
        let mut tx =
            InsecureAuthorizationTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("insecure_authorization"));
    }

    #[end]
    fn end(&mut self) {
        
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
