

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
        // Setup iniziale: crea l'utente e i metadata associati una volta sola.
        let mut init_user =
            InitializeUserTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut init_user, Some("initialize_user"));

        let mut init_metadata = InitializeUserMetadataTransaction::build(
            &mut self.trident,
            &mut self.fuzz_accounts,
        );
        self.trident
            .execute_transaction(&mut init_metadata, Some("initialize_user_metadata"));
    }

    #[flow]
    fn secure_user_read(&mut self) {
        // Esegue l'istruzione protetta di lettura utente con i dati fuzzati correnti.
        let mut tx =
            SecureUserReadTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("secure_user_read"));
    }

    #[end]
    fn end(&mut self) {
        
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
