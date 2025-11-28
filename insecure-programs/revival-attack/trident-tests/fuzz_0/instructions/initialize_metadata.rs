use crate::fuzz_accounts::FuzzAccounts; // pool condiviso di account controllati dal fuzzer
use borsh::{BorshDeserialize, BorshSerialize}; // codec Borsh per parametri segreti
use std::str::FromStr; // conversione da stringa a Pubkey
use trident_fuzz::fuzzing::*; // API generiche di Trident (Trident, PdaSeeds, ecc.)

#[derive(TridentInstruction, Default)]
#[program_id("BxYhDihgJZZxUXqwoqvzbfD8G1fwNFKQyF8L5SEiNCQP")]
#[discriminator([35u8, 215u8, 241u8, 156u8, 122u8, 208u8, 206u8, 212u8])]
pub struct InitializeMetadataInstruction {
    pub accounts: InitializeMetadataInstructionAccounts,
    pub data: InitializeMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeMetadataInstructionData)]
#[storage(FuzzAccounts)]
// account richiesti dall'istruzione di inizializzazione Anchor
pub struct InitializeMetadataInstructionAccounts {
    #[account(mut, signer)]
    // creatore che finanzia la PDA e diventa proprietario dei metadati
    pub creator: TridentAccount,

    #[account(mut)]
    // PDA dei metadati che ospita i segreti
    pub metadata: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    // programma di sistema utilizzato da Anchor::init
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
// quattro byte di PIN generati dal fuzzer
pub struct InitializeMetadataInstructionData {
    pub secret1: u8,

    pub secret2: u8,

    pub secret3: u8,

    pub secret4: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeMetadataInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        // Genera un PIN totalmente casuale; la chiusura successiva lo azzererà.
        self.data.secret1 = trident.gen_range(0..=u8::MAX);
        self.data.secret2 = trident.gen_range(0..=u8::MAX);
        self.data.secret3 = trident.gen_range(0..=u8::MAX);
        self.data.secret4 = trident.gen_range(0..=u8::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "BxYhDihgJZZxUXqwoqvzbfD8G1fwNFKQyF8L5SEiNCQP";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid revival attack program id");

        
        // Seleziona il creatore dal pool e lo finanzia con lamport di sicurezza.
        let creator_id = self.accounts.creator.account_id;
        let creator_pubkey = fuzz_accounts
            .creator
            .get_or_create(creator_id, trident, None, None);
        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();
        self.accounts.creator.set_is_writable();
        trident.airdrop(&creator_pubkey, 2 * LAMPORTS_PER_SOL);

        
        // Calcola il PDA deterministico e lo rende scrivibile per l'inizializzazione.
        let seeds: [&[u8]; 2] = [b"secret_metadata", creator_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(&seeds, &program_id);

        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();

        
        // Resetta l'account per permettere più iterazioni del fuzzer senza errori di init.
        let default_account = AccountSharedData::new(0, 0, &solana_sdk::system_program::ID);
        trident
            .get_client()
            .set_account_custom(&metadata_pubkey, &default_account);

        
        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);

        
        // Salva la mappatura nel pool per consentire a istruzioni successive di riferirsi allo stesso PDA.
        fuzz_accounts.metadata.get_or_create(
            self.accounts.metadata.account_id,
            trident,
            Some(PdaSeeds::new(&seeds, program_id)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );
    }
}
