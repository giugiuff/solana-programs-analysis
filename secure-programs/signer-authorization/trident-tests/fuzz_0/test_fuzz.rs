

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
            // Inizializza l'escrow solo alla prima iterazione per avere uno stato coerente.
            let mut tx = InitializeTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
            self.trident
                .execute_transaction(&mut tx, Some("initialize"));
            self.fuzz_accounts.escrow_initialized = true;
        }
    }

    #[flow]
    fn flow_secure_authorization(&mut self) {
        // Estrae casualmente se chiamare come attaccante o come utente legittimo.
        self.fuzz_accounts.call_as_attacker = self.trident.gen_range(0..2) == 1;

        // Esegue l'istruzione secure_authorization con i parametri correnti.
        let mut tx =
            SecureAuthorizationTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("secure_authorization"));

        self.fuzz_accounts.call_as_attacker = false;
    }

    #[end]
    fn end(&mut self) {}
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
