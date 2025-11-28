use crate::fuzz_accounts::FuzzAccounts; 
use borsh::{BorshDeserialize, BorshSerialize}; 
use solana_sdk::account::AccountSharedData; 
use solana_sdk::hash::hash; // Utility per generare hash necessari al discriminatore Anchor.
use solana_sdk::pubkey::Pubkey; // Tipo base per chiavi pubbliche Solana.
use solana_sdk::rent::Rent; // Calcolo del minimo di lamport per mantenere gli account.
use spl_token::solana_program::program_option::COption; // Wrapper opzionale usato nelle struct SPL.
use spl_token::solana_program::program_pack::Pack; // Trait per serializzare/deserializzare account SPL.
use spl_token::solana_program::pubkey::Pubkey as SplTokenPubkey; // Alias per Pubkey nel contesto SPL.
use spl_token::state::Account as SplTokenAccount; // Struttura che modella un token account SPL.
use spl_token::state::AccountState; // Stato di inizializzazione dell'account SPL.
use spl_token::state::Mint as SplMint; // Struttura che modella la mint SPL.
use std::str::FromStr; 
use trident_fuzz::fuzzing::*; // Macro e tipi principali del framework di fuzzing Trident.

#[derive(TridentInstruction, Default)]
#[program_id("4hKSDzDDxaHcdCDvULi2i5hHUsQ5oiS9NFx3uL1qtnoc")]
#[discriminator([208u8, 130u8, 53u8, 56u8, 11u8, 169u8, 199u8, 216u8])]
pub struct InsecureWithdrawInstruction {
    pub accounts: InsecureWithdrawInstructionAccounts,
    pub data: InsecureWithdrawInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InsecureWithdrawInstructionData)]
#[storage(FuzzAccounts)]
// Mappa gli account richiesti dall'istruzione Anchor verso gli slot gestiti da Trident.
pub struct InsecureWithdrawInstructionAccounts {
    #[account(
        signer,
        storage::name = creator,
        storage::account_id = 1,
    )]
    // Account dell'attaccante utilizzato come firmatario della CPI.
    pub creator: TridentAccount,

    #[account(
        mut,
        storage::name = vault,
        storage::account_id = 0,
    )]
    // ATA della vittima che funge da vault da cui prelevare i fondi.
    pub vault: TridentAccount,

    #[account(
        mut,
        storage::name = withdraw_destination,
        storage::account_id = 1,
    )]
    // ATA di destinazione controllato dall'attaccante.
    pub withdraw_destination: TridentAccount,

    #[account(
        storage::name = metadata_account,
        storage::account_id = 0,
    )]
    // PDA dei metadati che agisce da authority del vault.
    pub metadata_account: TridentAccount,

    #[account(
        storage::name = mint,
        storage::account_id = 0,
    )]
    // Mint SPL condivisa fra vittima e attaccante.
    pub mint: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    // Programma token ufficiale usato per la CPI.
    pub token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
// Nessun payload aggiuntivo: l'istruzione usa solo gli account.
pub struct InsecureWithdrawInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
// Hook Trident che prepara uno scenario di prelievo fraudolento dal vault.
impl InstructionHooks for InsecureWithdrawInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Programmi necessari per calcolare PDAs e creare account SPL.
        let program_id = program_pubkey();
        let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
            .expect("valid token program id");
        let ata_program = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
            .expect("valid associated token program id");

        // Firmatario attaccante che proverà a prelevare fondi non suoi.
        let attacker_pubkey = fuzz_accounts.creator.get_or_create(
            self.accounts.creator.account_id,
            trident,
            None,
            None,
        );
        self.accounts.creator.set_address(attacker_pubkey);
        self.accounts.creator.set_is_signer();

        // Proprietario legittimo del vault che dovrebbe rimanere protetto.
        let victim_pubkey = fuzz_accounts
            .vault_creator
            .get_or_create(0, trident, None, None);

        // Crea (o recupera) la mint condivisa che alimenterà i token del vault.
        let mint_pubkey = fuzz_accounts.mint.get_or_create_mint_account(
            self.accounts.mint.account_id,
            trident,
            None,
            0,
            &token_program,
            None,
        );
        self.accounts.mint.set_address(mint_pubkey);
        let vault_balance = trident.gen_range(1..=1_000_000_000u64);
        ensure_mint_account(
            trident,
            &mint_pubkey,
            &token_program,
            &victim_pubkey,
            vault_balance,
        );

        // Calcola il PDA di metadata registrato durante l'inizializzazione.
        let metadata_seeds: &[&[u8]] = &[b"metadata_account", victim_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(metadata_seeds, &program_id);
        self.accounts.metadata_account.set_address(metadata_pubkey);
        self.accounts.metadata_account.set_is_writable();

        ensure_metadata_account(trident, &metadata_pubkey, &victim_pubkey, &program_id);

        // PDA dell'ATA del vault della vittima, creato dall'istruzione di setup.
        let vault_seeds: &[&[u8]] = &[
            metadata_pubkey.as_ref(),
            token_program.as_ref(),
            mint_pubkey.as_ref(),
        ];
        let (vault_pubkey, _) = Pubkey::find_program_address(vault_seeds, &ata_program);
        self.accounts.vault.set_address(vault_pubkey);
        self.accounts.vault.set_is_writable();

        // Carica il vault con un saldo casuale per mettere in evidenza il bug di prelievo.
        ensure_token_account(
            trident,
            &vault_pubkey,
            &metadata_pubkey,
            mint_pubkey,
            &token_program,
            vault_balance,
        );

        // Account di destinazione dell'attaccante, inizialmente vuoto.
        let withdraw_pubkey = fuzz_accounts
            .withdraw_destination
            .get_or_create_token_account(
                self.accounts.withdraw_destination.account_id,
                trident,
                None,
                mint_pubkey,
                attacker_pubkey,
                0,
                None,
                0,
                None,
            );
        self.accounts
            .withdraw_destination
            .set_address(withdraw_pubkey);
        self.accounts.withdraw_destination.set_is_writable();
        ensure_token_account(
            trident,
            &withdraw_pubkey,
            &attacker_pubkey,
            mint_pubkey,
            &token_program,
            0,
        );
    }
}

// Restituisce l'ID del programma vulnerabile sottoposto a fuzzing.
fn program_pubkey() -> Pubkey {
    Pubkey::from_str("4hKSDzDDxaHcdCDvULi2i5hHUsQ5oiS9NFx3uL1qtnoc").expect("valid program id")
}

// Garantisce che l'account di metadata abbia il layout atteso con l'autorità corretta.
fn ensure_metadata_account(
    trident: &mut Trident,
    metadata_pubkey: &Pubkey,
    creator: &Pubkey,
    program_id: &Pubkey,
) {
    const METADATA_DATA_LEN: usize = 8 + 32;
    let mut metadata_account = trident.get_client().get_account(metadata_pubkey);
    if metadata_account.data().len() == METADATA_DATA_LEN {
        return;
    }

    let lamports = Rent::default().minimum_balance(METADATA_DATA_LEN);
    let mut metadata_bytes = Vec::with_capacity(METADATA_DATA_LEN);
    metadata_bytes.extend_from_slice(&account_discriminator("MetadataAccount"));
    metadata_bytes.extend_from_slice(creator.as_ref());

    metadata_account = AccountSharedData::new(lamports, METADATA_DATA_LEN, program_id);
    metadata_account.set_executable(false);
    metadata_account
        .data_as_mut_slice()
        .copy_from_slice(&metadata_bytes);

    trident
        .get_client()
        .set_account_custom(metadata_pubkey, &metadata_account);
}

// Inizializza o aggiorna un token account SPL con owner, mint e saldo desiderati.
fn ensure_token_account(
    trident: &mut Trident,
    account_pubkey: &Pubkey,
    authority: &Pubkey,
    mint: Pubkey,
    token_program: &Pubkey,
    amount: u64,
) {
    // Recupera lo stato corrente dell'ATA; se non esiste lo ricrea da zero.
    let mut token_account = trident.get_client().get_account(account_pubkey);
    if token_account.data().len() != SplTokenAccount::LEN || token_account.owner() != token_program
    {
        let rent = Rent::default().minimum_balance(SplTokenAccount::LEN);
        token_account = AccountSharedData::new(rent, SplTokenAccount::LEN, token_program);
        token_account.set_executable(false);

        let mint_key = SplTokenPubkey::new_from_array(mint.to_bytes());
        let authority_key = SplTokenPubkey::new_from_array(authority.to_bytes());
        let token_state = SplTokenAccount {
            mint: mint_key,
            owner: authority_key,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        };
        let mut account_bytes = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(token_state, &mut account_bytes).unwrap();
        token_account
            .data_as_mut_slice()
            .copy_from_slice(&account_bytes);

        trident
            .get_client()
            .set_account_custom(account_pubkey, &token_account);
    }

    // Aggiorna l'ammontare mantenendo invariata la restante struttura dell'account.
    let mut token_state = SplTokenAccount::unpack(token_account.data()).expect("valid ATA");
    token_state.amount = amount;
    let mut account_bytes = vec![0u8; SplTokenAccount::LEN];
    SplTokenAccount::pack(token_state, &mut account_bytes).unwrap();
    token_account.set_data_from_slice(&account_bytes);
    trident
        .get_client()
        .set_account_custom(account_pubkey, &token_account);
}

fn ensure_mint_account(
    trident: &mut Trident,
    mint_pubkey: &Pubkey,
    token_program: &Pubkey,
    mint_authority: &Pubkey,
    supply: u64,
) {
    // Recupera o crea la mint SPL condivisa tra le parti coinvolte.
    let mut mint_account = trident.get_client().get_account(mint_pubkey);
    if mint_account.data().len() != SplMint::LEN || mint_account.owner() != token_program {
        let rent = Rent::default().minimum_balance(SplMint::LEN);
        mint_account = AccountSharedData::new(rent, SplMint::LEN, token_program);
        mint_account.set_executable(false);
    }

    // Serializza lo stato della mint impostando supply, authority e flag di inizializzazione.
    let authority_key = SplTokenPubkey::new_from_array(mint_authority.to_bytes());
    let mint_state = SplMint {
        mint_authority: COption::Some(authority_key),
        supply,
        decimals: 0,
        is_initialized: true,
        freeze_authority: COption::None,
    };
    let mut mint_bytes = vec![0u8; SplMint::LEN];
    SplMint::pack(mint_state, &mut mint_bytes).unwrap();
    mint_account
        .data_as_mut_slice()
        .copy_from_slice(&mint_bytes);
    trident
        .get_client()
        .set_account_custom(mint_pubkey, &mint_account);
}

fn account_discriminator(name: &str) -> [u8; 8] {
    // Riproduce il calcolo Anchor del discriminatore per un determinato account.
    let hash = hash(format!("account:{name}").as_bytes());
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash.to_bytes()[..8]);
    discriminator
}
