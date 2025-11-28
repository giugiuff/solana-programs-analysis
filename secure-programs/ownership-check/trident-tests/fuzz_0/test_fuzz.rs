// Registers the Ownership Check fuzz suite with Trident and drives the generated transactions during fuzzing.

use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use types::OwnershipScenario;

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
    fn flow1(&mut self) {
        self.fuzz_accounts.ownership = generate_ownership_scenario(&mut self.trident);

        let mut tx =
            SecureLogBalanceV1Transaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("secure_log_balance_v1"));
    }

    #[flow]
    fn flow2(&mut self) {
        // Esegue la seconda variante.
        let mut tx =
            SecureLogBalanceV2Transaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx, Some("secure_log_balance_v2"));
    }
    


    #[end]
    fn end(&mut self) {}
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}

fn generate_ownership_scenario(trident: &mut Trident) -> OwnershipScenario {
    let use_correct_owner = trident.gen_range(0..100_u8) < 50;

    OwnershipScenario {
        use_correct_owner,
        ..OwnershipScenario::default()
    }
}
