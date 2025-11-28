// Sets up the secure_withdraw instruction for the PDA Privileges fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::account::AccountSharedData;
use solana_sdk::hash::hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use spl_token::solana_program::program_option::COption;
use spl_token::solana_program::program_pack::Pack;
use spl_token::solana_program::pubkey::Pubkey as SplTokenPubkey;
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplMint};
use std::str::FromStr;
use trident_fuzz::fuzzing::solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("J6SWoen6bHQQEnPpU8TUiU46ABpthjcvrYeiXCcUXa17")]
#[discriminator([16u8, 104u8, 17u8, 169u8, 118u8, 59u8, 103u8, 42u8])]
pub struct SecureWithdrawInstruction {
    pub accounts: SecureWithdrawInstructionAccounts,
    pub data: SecureWithdrawInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SecureWithdrawInstructionData)]
#[storage(FuzzAccounts)]
pub struct SecureWithdrawInstructionAccounts {
    #[account(signer)]
    pub creator: TridentAccount,

    #[account(mut)]
    pub vault: TridentAccount,

    #[account(mut)]
    pub withdraw_destination: TridentAccount,

    pub metadata_account: TridentAccount,

    pub mint: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SecureWithdrawInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for SecureWithdrawInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Scenario generato dal flow: contiene gli ID per account legit/attacker e flag di correttezza.
        let scenario = &fuzz_accounts.withdraw;

        // Program IDs fissi usati per derivare PDA e creare account SPL.
        let program_id = program_pubkey();
        let token_program = token_program_pubkey();
        let ata_program = associated_token_program_pubkey();

        // Recuperiamo (o creiamo) il creatore legittimo e lo finanziamo per poter firmare.
        let legit_creator = fuzz_accounts.vault_creator.get_or_create(
            scenario.legit_creator_id,
            trident,
            None,
            None,
        );
        let legit_creator_pubkey = Pubkey::new_from_array(legit_creator.to_bytes());
        trident.airdrop(&legit_creator, 5 * LAMPORTS_PER_SOL);

        // Anche l'attaccante riceve un account per simulare firme e destinazioni maliziose.
        let attacker_creator =
            fuzz_accounts
                .creator
                .get_or_create(scenario.attacker_creator_id, trident, None, None);
        let attacker_creator_pubkey = Pubkey::new_from_array(attacker_creator.to_bytes());
        trident.airdrop(&attacker_creator, 5 * LAMPORTS_PER_SOL);

        // A seconda dello scenario scegliamo quale creatore firmi la transazione.
        let (signer_pubkey, signer_account_id) = if scenario.use_correct_creator {
            (legit_creator, scenario.legit_creator_id)
        } else {
            (attacker_creator, scenario.attacker_creator_id)
        };

        // Configuriamo il campo creator dell'istruzione e lo marchiamo come signer.
        self.accounts.creator.set_address(signer_pubkey);
        self.accounts.creator.set_is_signer();
        self.accounts.creator.account_id = signer_account_id;

        // Creiamo la mint se manca, allocando i lamport necessari via AccountMetadata.
        let mint_pubkey = fuzz_accounts.mint.get_or_create(
            scenario.mint_account_id,
            trident,
            None,
            Some(AccountMetadata::new(
                Rent::default().minimum_balance(SplMint::LEN),
                SplMint::LEN,
                token_program,
            )),
        );
        self.accounts.mint.set_address(mint_pubkey);
        self.accounts.mint.set_is_writable();
        self.accounts.mint.account_id = scenario.mint_account_id;

        // Metadata PDA è derivata dal creatore legittimo (come nel programma on-chain).
        let metadata_seeds: &[&[u8]] = &[b"metadata_account", legit_creator_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(metadata_seeds, &program_id);
        self.accounts.metadata_account.set_address(metadata_pubkey);
        self.accounts.metadata_account.set_is_writable();
        self.accounts.metadata_account.account_id = scenario.metadata_account_id;

        // Assicuriamoci che l'account metadata esista con il creatore corretto.
        ensure_metadata_account(
            trident,
            &metadata_pubkey,
            &legit_creator_pubkey,
            &program_id,
        );

        // Il vault è l'ATA di metadata_account, usato come sorgente token.
        let vault_seeds: &[&[u8]] = &[
            metadata_pubkey.as_ref(),
            token_program.as_ref(),
            mint_pubkey.as_ref(),
        ];
        let (vault_pubkey, _) = Pubkey::find_program_address(vault_seeds, &ata_program);
        self.accounts.vault.set_address(vault_pubkey);
        self.accounts.vault.set_is_writable();
        self.accounts.vault.account_id = scenario.vault_account_id;

        // Simuliamo un saldo casuale per rendere interessante il prelievo.
        let vault_balance = trident.gen_range(1..=1_000_000_000_u64);

        // Ci assicuriamo che la mint e l'ATA (associated token account) del vault siano valorizzati come previsto.
        ensure_mint_account(
            trident,
            &mint_pubkey,
            &token_program,
            &legit_creator_pubkey,
            vault_balance,
        );

        ensure_token_account(
            trident,
            &vault_pubkey,
            &metadata_pubkey,
            mint_pubkey,
            &token_program,
            vault_balance,
        );

        // Anche la destinazione dipende dal flag: o account legit o attaccante.
        let (withdraw_owner_pubkey, withdraw_account_id) = if scenario.use_correct_creator {
            (legit_creator_pubkey, scenario.legit_destination_id)
        } else {
            (attacker_creator_pubkey, scenario.attacker_destination_id)
        };

        let withdraw_seeds: &[&[u8]] = &[
            withdraw_owner_pubkey.as_ref(),
            token_program.as_ref(),
            mint_pubkey.as_ref(),
        ];
        let (withdraw_pubkey, _) = Pubkey::find_program_address(withdraw_seeds, &ata_program);
        self.accounts
            .withdraw_destination
            .set_address(withdraw_pubkey);
        self.accounts.withdraw_destination.set_is_writable();
        self.accounts.withdraw_destination.account_id = withdraw_account_id;

        // Garantiamo che l'ATA di destinazione esista (saldo iniziale zero).
        ensure_token_account(
            trident,
            &withdraw_pubkey,
            &withdraw_owner_pubkey,
            mint_pubkey,
            &token_program,
            0,
        );

        // Token program costante richiesto dall'istruzione Anchor.
        self.accounts.token_program.set_address(token_program);
    }
}

fn ensure_metadata_account(
    trident: &mut Trident,
    metadata_pubkey: &Pubkey,
    creator: &Pubkey,
    program_id: &Pubkey,
) {
    const METADATA_DATA_LEN: usize = 8 + 32;
    let mut metadata_account = trident.get_client().get_account(metadata_pubkey);
    if metadata_account.data().len() == METADATA_DATA_LEN {
        let current_creator = &metadata_account.data()[8..40];
        if current_creator == creator.as_ref() {
            return;
        }
    }

    let lamports = Rent::default().minimum_balance(METADATA_DATA_LEN);
    metadata_account = AccountSharedData::new(lamports, METADATA_DATA_LEN, program_id);
    metadata_account.set_executable(false);

    let mut metadata_bytes = Vec::with_capacity(METADATA_DATA_LEN);
    metadata_bytes.extend_from_slice(&account_discriminator("MetadataAccount"));
    metadata_bytes.extend_from_slice(creator.as_ref());
    metadata_account
        .data_as_mut_slice()
        .copy_from_slice(&metadata_bytes);

    trident
        .get_client()
        .set_account_custom(metadata_pubkey, &metadata_account);
}

fn ensure_token_account(
    trident: &mut Trident,
    account_pubkey: &Pubkey,
    authority: &Pubkey,
    mint: Pubkey,
    token_program: &Pubkey,
    amount: u64,
) {
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

    let mut token_state = SplTokenAccount::unpack(token_account.data()).expect("valid token");
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
    let mut mint_account = trident.get_client().get_account(mint_pubkey);
    if mint_account.data().len() != SplMint::LEN || mint_account.owner() != token_program {
        let rent = Rent::default().minimum_balance(SplMint::LEN);
        mint_account = AccountSharedData::new(rent, SplMint::LEN, token_program);
        mint_account.set_executable(false);
    }

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
    let hash = hash(format!("account:{name}").as_bytes());
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash.to_bytes()[..8]);
    discriminator
}

fn program_pubkey() -> Pubkey {
    Pubkey::from_str("J6SWoen6bHQQEnPpU8TUiU46ABpthjcvrYeiXCcUXa17").expect("valid program id")
}

fn token_program_pubkey() -> Pubkey {
    Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").expect("valid token program id")
}

fn associated_token_program_pubkey() -> Pubkey {
    Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")
        .expect("valid associated token program id")
}
