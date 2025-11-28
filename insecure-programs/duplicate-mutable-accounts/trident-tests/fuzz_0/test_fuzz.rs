use fuzz_accounts::*; // Porta nel namespace gli account condivisi definiti per il fuzzing.
use sha2::{Digest, Sha256}; // Utilizzato per calcolare il discriminator dell'account Vault.
use trident_fuzz::fuzzing::*; // Importa le macro e i tipi principali del framework Trident.
use trident_fuzz::fuzzing::solana_sdk::account::Account; // Rappresenta un account Solana grezzo.
use trident_fuzz::fuzzing::AccountSharedData; // Wrapper condiviso per lo stato degli account nel fuzzer.
use trident_fuzz::fuzzing::TridentPubkey; // Tipo helper per gestire chiavi pubbliche nel fuzzer.
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use types::{AtomicTradeScenario, Vault};

const PROGRAM_ID: Pubkey = pubkey!("2bSdoVHibWNGdDRZrd3bYJpSNyEMu2DRVpgPPajdoU24"); // ID del programma Anchor analizzato.
const VAULT_ACCOUNT_SPACE: usize = 8 + 32 + 8; // Discriminatore (8) + Pubkey (32) + amount (8).

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Motore Trident che coordina generazione di transazioni e dati casuali.
    trident: Trident,
    /// Archivio di account condivisi tra le varie istruzioni del fuzzer.
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
        // Genera uno scenario casuale (incluso se usare vault duplicati) e configura gli account coerentemente.
        self.fuzz_accounts.atomic_trade = generate_atomic_trade_scenario(&mut self.trident);
        prepare_atomic_trade_accounts(&mut self.trident, &mut self.fuzz_accounts);

        // Costruisce ed esegue la transazione `insecure_atomic_trade` con i dati appena preparati.
        let mut tx = InsecureAtomicTradeTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident
            .execute_transaction(&mut tx, Some("insecure_atomic_trade"));
    }


    #[end]
    fn end(&mut self) {}
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}

fn generate_atomic_trade_scenario(trident: &mut Trident) -> AtomicTradeScenario {
    // Decidi con il 50% di probabilità se i due vault dovranno coincidere (riproducendo la vulnerabilità).
    let duplicate_vaults = trident.gen_range(0..100_u8) < 50;
    // Importo trasferito e saldi iniziali dei vault, generati in modo casuale.
    let transfer_amount = trident.gen_range(1..1_000_u64);
    let vault_b_balance = transfer_amount + trident.gen_range(10..1_000_u64);
    let vault_a_balance = if duplicate_vaults {
        vault_b_balance
    } else {
        trident.gen_range(0..1_000_u64)
    };

    AtomicTradeScenario {
        duplicate_vaults,
        transfer_amount,
        vault_a_balance,
        vault_b_balance,
        fee_vault_balance: trident.gen_range(0..1_000_u64),
        ..AtomicTradeScenario::default()
    }
}

fn prepare_atomic_trade_accounts(trident: &mut Trident, fuzz_accounts: &mut FuzzAccounts) {
    let scenario = &fuzz_accounts.atomic_trade;

    // Crea o recupera il firmatario A e lo rifornisce di lamport per coprire i costi di transazione.
    let signer_a = fuzz_accounts
        .signer_a
        .get_or_create(scenario.signer_a_id, trident, None, None);
    trident.airdrop(&signer_a, 5 * LAMPORTS_PER_SOL);

    // Se si testa il caso vulnerabile, il firmatario B coincide con A; altrimenti se ne genera uno distinto.
    let signer_b = if scenario.duplicate_vaults {
        signer_a
    } else {
        let signer_b_pubkey = fuzz_accounts
            .signer_b
            .get_or_create(scenario.signer_b_id, trident, None, None);
        trident.airdrop(&signer_b_pubkey, 5 * LAMPORTS_PER_SOL);
        signer_b_pubkey
    };

    // Prepara l'autorità del vault delle commissioni e assegna fondi.
    let fee_authority = fuzz_accounts
        .authority
        .get_or_create(scenario.fee_authority_id, trident, None, None);
    trident.airdrop(&fee_authority, 5 * LAMPORTS_PER_SOL);

    // Calcola la PDA del vault A e crea/inizializza l'account con il saldo previsto dallo scenario.
    let signer_a_bytes = signer_a.to_bytes();
    let vault_a_seeds: [&[u8]; 2] = [b"vault", signer_a_bytes.as_ref()];
    let vault_a = fuzz_accounts.vault_a.get_or_create(
        scenario.vault_a_id,
        trident,
        Some(PdaSeeds::new(&vault_a_seeds, PROGRAM_ID)),
        Some(AccountMetadata::new(
            10 * LAMPORTS_PER_SOL,
            VAULT_ACCOUNT_SPACE,
            PROGRAM_ID,
        )),
    );
    let vault_a_account = make_vault_account(signer_a, scenario.vault_a_balance);
    trident
        .get_client()
        .set_account_custom(&vault_a, &vault_a_account);

    if scenario.duplicate_vaults {
        // Allinea entrambi i riferimenti per puntare allo stesso stato (riproduce l'errore in produzione).
        let vault_state = make_vault_account(signer_a, scenario.vault_b_balance);
        trident
            .get_client()
            .set_account_custom(&vault_a, &vault_state);
    } else {
        // Caso sano: crea un secondo vault con seeds basate sul firmatario B.
        let signer_b_bytes = signer_b.to_bytes();
        let vault_b_seeds: [&[u8]; 2] = [b"vault", signer_b_bytes.as_ref()];
        let vault_b = fuzz_accounts.vault_b.get_or_create(
            scenario.vault_b_id,
            trident,
            Some(PdaSeeds::new(&vault_b_seeds, PROGRAM_ID)),
            Some(AccountMetadata::new(
                10 * LAMPORTS_PER_SOL,
                VAULT_ACCOUNT_SPACE,
                PROGRAM_ID,
            )),
        );
        let vault_b_account = make_vault_account(signer_b, scenario.vault_b_balance);
        trident
            .get_client()
            .set_account_custom(&vault_b, &vault_b_account);
    }

    // Crea/aggiorna il vault delle commissioni con l'importo di partenza definito nello scenario.
    let fee_vault_seeds: [&[u8]; 1] = [b"fee_vault"];
    let fee_vault = fuzz_accounts.fee_vault.get_or_create(
        scenario.fee_vault_id,
        trident,
        Some(PdaSeeds::new(&fee_vault_seeds, PROGRAM_ID)),
        Some(AccountMetadata::new(
            10 * LAMPORTS_PER_SOL,
            VAULT_ACCOUNT_SPACE,
            PROGRAM_ID,
        )),
    );
    let fee_vault_account = make_vault_account(fee_authority, scenario.fee_vault_balance);
    trident
        .get_client()
        .set_account_custom(&fee_vault, &fee_vault_account);
}

fn make_vault_account(owner: Pubkey, amount: u64) -> AccountSharedData {
    // Converte la chiave pubblica in formato Trident e costruisce lo stato dell'account Vault.
    let mut owner_key = TridentPubkey::default();
    owner_key.set_pubkey(owner);

    let vault_state = Vault {
        owner: owner_key,
        amount,
    };

    // Serializza il discriminatore e i campi della struct per ottenere il payload dell'account Anchor.
    let mut data = Vec::with_capacity(VAULT_ACCOUNT_SPACE);
    data.extend_from_slice(&vault_discriminator());
    data.extend(
        borsh::to_vec(&vault_state).expect("serialize vault account state"),
    );

    // Restituisce un account condiviso con lamport sufficienti e assegnato al programma target.
    AccountSharedData::from(Account {
        lamports: 10 * LAMPORTS_PER_SOL,
        data,
        owner: PROGRAM_ID,
        executable: false,
        rent_epoch: 0,
    })
}

fn vault_discriminator() -> [u8; 8] {
    // Calcola il discriminatore Anchor dell'account Vault utilizzando SHA-256.
    let hash = Sha256::digest(b"account:Vault");
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash[..8]);
    discriminator
}
