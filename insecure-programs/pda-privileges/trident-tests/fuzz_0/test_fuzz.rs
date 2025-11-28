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
    /// Flag per inizializzare il vault una sola volta.
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
        // Inizializza il vault vittima una sola volta così i prelievi fuzzati lavorano su stato realistico.
        if !self.vault_bootstrap_done {
            let mut init_tx =
                InitializeVaultTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
            self.trident
                .execute_transaction(&mut init_tx, Some("initialize_vault"));
            self.vault_bootstrap_done = true;
        }
    }

    #[flow]
    fn insecure_withdraw(&mut self) {
        // Esecuzione del prelievo insicuro con parametri randomizzati.
        let mut tx = InsecureWithdrawTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("insecure_withdraw"));
    }

    #[end]
    fn end(&mut self) {
        
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
