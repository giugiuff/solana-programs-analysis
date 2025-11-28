use fuzz_accounts::*; 
use trident_fuzz::fuzzing::*; 
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Motore di esecuzione Trident che costruisce e invia le transazioni durante i test.
    trident: Trident,
    /// Stato condiviso fra le istruzioni che memorizza account e parametri generati.
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
        // Resetta l'eventuale PDA memorizzata in precedenza.
        self.fuzz_accounts.secret_pda = None;
        // Disattiva la modalità d'attacco di default.
        self.fuzz_accounts.attack_mode = false;
        // Genera un PIN casuale a quattro byte utilizzando il generatore integrato in Trident.
        self.fuzz_accounts.pins = [
            self.trident.gen_range(0..=u8::MAX),
            self.trident.gen_range(0..=u8::MAX),
            self.trident.gen_range(0..=u8::MAX),
            self.trident.gen_range(0..=u8::MAX),
        ];

        // Costruisce e invia la transazione di inizializzazione del PIN.
        let mut initialize =
            InitializeSecretTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut initialize, Some("initialize_secret"));
    }

    #[flow]
    fn flow1(&mut self) {
        // Esegue il flusso legittimo impostando la modalità d'attacco a false.
        self.fuzz_accounts.attack_mode = false;
        let mut tx =
            InsecureVerifyPinTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("insecure_verify_legit"));
    }

    #[flow]
    fn flow2(&mut self) {
        // Abilita la modalità d'attacco per simulare un CPI verso un programma non autorizzato.
        self.fuzz_accounts.attack_mode = true;
        let mut tx =
            InsecureVerifyPinTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("insecure_verify_hacked"));
        self.fuzz_accounts.attack_mode = false;
    }

    #[end]
    fn end(&mut self) { 
    }
}

fn main() {
    
    FuzzTest::fuzz(1000, 100);
}
