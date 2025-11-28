use fuzz_accounts::*; // Importa gli account condivisi definiti per gli scenari di fuzzing.
use trident_fuzz::fuzzing::*; // Porta nel namespace macro e tipi principali di Trident.
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Motore Trident che genera invocazioni e parametri casuali.
    trident: Trident,
    /// Archivio condiviso di account resi disponibili ai vari hook.
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
        // Punto di estensione: eseguire qui eventuali setup prima di ogni iterazione di fuzzing.
    }

    #[flow]
    fn flow1(&mut self) {
        // Esegue la prima variante dell'istruzione: il controllo fallisce quando il proprietario reale diverge.
        let mut tx =
            InsecureLogBalanceV1Transaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx, Some("insecure_log_balance_v1"));
    }

    #[flow]
    fn flow2(&mut self) {
        // Esegue la seconda variante.
        let mut tx =
            InsecureLogBalanceV2Transaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx, Some("insecure_log_balance_v2"));
    }

    #[end]
    fn end(&mut self) {
        // Punto di estensione per pulizie dopo ogni iterazione; lasciato vuoto perché non necessario.
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
