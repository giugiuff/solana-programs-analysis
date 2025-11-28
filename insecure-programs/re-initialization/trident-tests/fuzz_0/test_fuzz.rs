use borsh::to_vec; // Serializza la struct Metadata in formato Borsh.
use fuzz_accounts::*; // Espone lo storage condiviso con tutti gli hook.
use sha2::{Digest, Sha256}; // Usato per calcolare il discriminatore Anchor dell'account.
use trident_fuzz::fuzzing::AccountSharedData; // Rappresenta lo stato di un account lato client Trident.
use trident_fuzz::fuzzing::*; // Macro e API principali del framework Trident.
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use types::{ATTACKER_ID, INITIAL_LAMPORTS, LEGIT_CREATOR_ID, METADATA_ACCOUNT_SPACE, PROGRAM_ID};

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Esecutore Trident utilizzato da tutti i flussi di fuzzing.
    trident: Trident,
    /// Archivio di supporto che mantiene gli account creati durante i test.
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
        // Prepara in anticipo i firmatari deterministici (legittimo + attaccante).
        self.prepare_signers();
    }

    #[flow]
    fn insecure_initialize_v1_flow(&mut self) {
        // Rigenera il PDA dei metadati, esegue l'init legittimo e poi la re-init dell'attaccante.
        self.reset_metadata_account();
        self.run_insecure_v1(false);
        self.run_insecure_v1(true);
    }

    #[flow]
    fn insecure_initialize_v2_flow(&mut self) {
        // Stessa sequenza per l'entrypoint v2, così da mostrare la medesima vulnerabilità.
        self.reset_metadata_account();
        self.run_insecure_v2(false);
        self.run_insecure_v2(true);
    }

    #[end]
    fn end(&mut self) {}
}

impl FuzzTest {
    fn prepare_signers(&mut self) {
        // Deriva chiavi deterministiche per creatore e attaccante.
        let legit = self.fuzz_accounts.creator.get_or_create(
            LEGIT_CREATOR_ID,
            &mut self.trident,
            None,
            None,
        );
        let attacker =
            self.fuzz_accounts
                .attacker
                .get_or_create(ATTACKER_ID, &mut self.trident, None, None);

        // Finanzia entrambi per coprire la rent delle transazioni successive.
        self.trident.airdrop(&legit, INITIAL_LAMPORTS);
        self.trident.airdrop(&attacker, INITIAL_LAMPORTS);
    }

    fn reset_metadata_account(&mut self) {
        // Ricrea il PDA dei metadati con dati di default prima di ogni sequenza.
        let (metadata_pubkey, _) = Pubkey::find_program_address(&[b"metadata"], &PROGRAM_ID);

        let mut data = Vec::with_capacity(METADATA_ACCOUNT_SPACE);
        data.extend_from_slice(&metadata_discriminator());
        data.extend(to_vec(&types::Metadata::default()).expect("serialize metadata state"));
        data.resize(METADATA_ACCOUNT_SPACE, 0);

        let mut account =
            AccountSharedData::new(INITIAL_LAMPORTS, METADATA_ACCOUNT_SPACE, &PROGRAM_ID);
        account.set_data_from_slice(&data);
        self.trident
            .get_client()
            .set_account_custom(&metadata_pubkey, &account);
    }

    fn run_insecure_v1(&mut self, as_attacker: bool) {
        // Cambia persona, esegue l'inizializzatore v1, poi ripristina il flag.
        self.fuzz_accounts.call_as_attacker = as_attacker;
        let mut tx =
            InsecureInitializev1Transaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("insecure_initialize_v1"));
        self.fuzz_accounts.call_as_attacker = false;
    }

    fn run_insecure_v2(&mut self, as_attacker: bool) {
        // Ripete la stessa logica per l'inizializzatore v2.
        self.fuzz_accounts.call_as_attacker = as_attacker;
        let mut tx =
            InsecureInitializev2Transaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("insecure_initialize_v2"));
        self.fuzz_accounts.call_as_attacker = false;
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}

fn metadata_discriminator() -> [u8; 8] {
    // Calcola il discriminatore Anchor per l'account Metadata (hash del nome).
    let mut hasher = Sha256::new();
    hasher.update(b"account:Metadata");
    let hash = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash[..8]);
    discriminator
}
