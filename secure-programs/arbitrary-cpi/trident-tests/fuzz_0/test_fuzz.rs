// Registers the Arbitrary CPI fuzz suite with Trident and drives the generated transactions during fuzzing.

use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// for fuzzing
    trident: Trident,
    /// for storing fuzzing accounts
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
        self.fuzz_accounts.secret_pda = None;
        self.fuzz_accounts.attack_mode = false;

        let mut init_tx =
            InitializeSecretTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut init_tx, Some("initialize_secret"));
    }

    #[flow]
    fn flow1(&mut self) {
        self.fuzz_accounts.attack_mode = false;
        let mut tx =
            SecureVerifyPinTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("secure_verify_legit"));
    }

    #[flow]
    fn flow2(&mut self) {
        self.fuzz_accounts.attack_mode = true;
        let mut tx =
            SecureVerifyPinTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("secure_verify_hacked"));
        self.fuzz_accounts.attack_mode = false;
    }

    #[end]
    fn end(&mut self) {
        // perform any cleaning here, this method will be executed
        // at the end of each iteration
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
