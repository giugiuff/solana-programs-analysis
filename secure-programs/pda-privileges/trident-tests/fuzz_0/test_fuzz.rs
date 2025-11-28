

use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use types::WithdrawScenario;


#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Runtime Trident che eroga chiavi, lamport e CPI al programma.
    trident: Trident,
    /// Istanza degli account creati e riutilizzati durante i test.
    fuzz_accounts: FuzzAccounts,
    vault_bootstrap_done: bool,
}


#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: FuzzAccounts::default(),
            vault_bootstrap_done: false,
        }
    }

    #[init]
    fn start(&mut self) {
        // All'avvio creiamo il vault una sola volta così i test successivi partono da uno stato valido.
        if !self.vault_bootstrap_done {
            let mut init_tx =
                InitializeVaultTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
            self.trident
                .execute_transaction(&mut init_tx, Some("initialize_vault"));
            self.vault_bootstrap_done = true;
        }
    }

    #[flow]
    fn flow_secure_withdraw(&mut self) {
        // Costruiamo uno scenario di prelievo casuale e lo eseguiamo contro il programma.
        self.fuzz_accounts.withdraw = generate_withdraw_scenario(&mut self.trident);

        let mut tx = SecureWithdrawTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("secure_withdraw"));
    }

    #[end]
    fn end(&mut self) {
        
    }
}

fn main() {
    // Avvia il fuzzing per 1000 iterazioni, con 100 chiamate di flow per iterazione.
    FuzzTest::fuzz(1000, 100);
}

fn generate_withdraw_scenario(trident: &mut Trident) -> WithdrawScenario {
    let use_correct_creator = trident.gen_range(0..100_u8) < 50;

    WithdrawScenario {
        use_correct_creator,
        ..WithdrawScenario::default()
    }
}
