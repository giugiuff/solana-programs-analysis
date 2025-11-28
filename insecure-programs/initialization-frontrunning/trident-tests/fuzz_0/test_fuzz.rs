use fuzz_accounts::*; // Importa gli account condivisi definiti nella cartella fuzz_0.
use trident_fuzz::fuzzing::solana_sdk::account::AccountSharedData; // Wrapper per manipolare lo stato degli account nel test.
use trident_fuzz::fuzzing::solana_sdk::pubkey::Pubkey; // Tipo Pubkey di Solana usato nelle PDA.
use trident_fuzz::fuzzing::solana_sdk::system_program; // Programma di sistema richiesto per l'inizializzazione.
use trident_fuzz::fuzzing::*; // Porta nel namespace le macro e le utilità principali di Trident.
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use types::{InitializationScenario, ATTACKER_SIGNER_ID, LEGIT_SIGNER_ID};

const PROGRAM_ID: Pubkey = pubkey!("EcaXg5bCYZsWjAM7wZ1xY3E7Mp95bYbur2WC5NfqpRjw"); // ID del programma vulnerabile che vogliamo testare.

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Motore Trident responsabile di generare transazioni e dati casuali.
    trident: Trident,
    /// Stato condiviso tra le istruzioni che conserva account e scenari di test.
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
    fn flow_initialize_insecure(&mut self) {
        // Sorteggia lo scenario: metà delle volte usa l'autorità corretta, altrimenti l'attaccante.
        self.fuzz_accounts.initialization = generate_initialization_scenario(&mut self.trident);
        // Prepara account e PDA coerenti con la scelta precedente.
        prepare_initialization_accounts(&mut self.trident, &mut self.fuzz_accounts);

        // Costruisce ed esegue la transazione di inizializzazione sul programma da analizzare.
        let mut tx =
            InitializeInsecureTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("initialize_insecure"));
    }

    #[end]
    fn end(&mut self) {}
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}

fn generate_initialization_scenario(trident: &mut Trident) -> InitializationScenario {
    // Probabilità 50% di usare il firmatario legittimo, 50% l'attaccante.
    let use_correct_authority = trident.gen_range(0..100_u8) < 50;
    // Dato accessorio casuale che finirà nella configurazione globale.
    let additional_data = trident.gen_range(0..=u8::MAX);

    InitializationScenario {
        use_correct_authority,
        additional_data,
        ..InitializationScenario::default()
    }
}

fn prepare_initialization_accounts(trident: &mut Trident, fuzz_accounts: &mut FuzzAccounts) {
    let scenario = &fuzz_accounts.initialization;

    // Genera o recupera l'account del legittimo proprietario e quello dell'attaccante.
    let legit_signer = fuzz_accounts
        .signer
        .get_or_create(LEGIT_SIGNER_ID, trident, None, None);
    let attacker_signer =
       fuzz_accounts
           .signer
           .get_or_create(ATTACKER_SIGNER_ID, trident, None, None);

    // Airdrop per coprire i costi di transazione di entrambi gli attori dello scenario.
    trident.airdrop(&legit_signer, 5 * LAMPORTS_PER_SOL);
    trident.airdrop(&attacker_signer, 5 * LAMPORTS_PER_SOL);

    // Crea (o recupera) il PDA `global_config` con seed fisso, riproducendo la situazione vulnerabile.
    let global_config = fuzz_accounts.global_config.get_or_create(
       scenario.global_config_id,
       trident,
       Some(PdaSeeds::new(&[b"config"], PROGRAM_ID)),
       None,
   );

    // Imposta lo stato iniziale dell'account come vuoto e assegnato al programma di sistema,
    // in modo che l'istruzione Anchor sia costretta a inizializzarlo.
    let global_config_account = AccountSharedData::new(0, 0, &system_program::ID);
    trident
        .get_client()
        .set_account_custom(&global_config, &global_config_account);
}
