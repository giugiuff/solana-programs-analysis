use crate::fuzz_accounts::FuzzAccounts; // archivio condiviso di account e flag d'inizializzazione
use crate::types::*; // struct Borsh `UserMetadata` usata nei controlli/invariants
use borsh::{BorshDeserialize, BorshSerialize}; // codec per i parametri
use solana_sdk::pubkey::Pubkey; // per calcolare PDAs
use std::str::FromStr; // conversione del program id in Pubkey
use trident_fuzz::fuzzing::*; // API generiche di Trident

#[derive(TridentInstruction, Default)]
#[program_id("5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618")]
#[discriminator([150u8, 144u8, 210u8, 183u8, 236u8, 161u8, 77u8, 76u8])]
pub struct InitializeUserMetadataInstruction {
    pub accounts: InitializeUserMetadataInstructionAccounts,
    pub data: InitializeUserMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeUserMetadataInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeUserMetadataInstructionAccounts {
    #[account(mut, signer)]
    pub authority: TridentAccount,

    #[account(mut)]
    pub user_metadata: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeUserMetadataInstructionData {
    pub user_account: TridentPubkey,

    pub pin1: u8,

    pub pin2: u8,

    pub pin3: u8,

    pub pin4: u8,
}

impl InstructionHooks for InitializeUserMetadataInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        // Preleva l'autorità legittima per derivare PDAs coerenti.
        let authority_pubkey = fuzz_accounts.authority.get_or_create(
            self.accounts.authority.account_id,
            trident,
            None,
            None,
        );

        // Calcola il PDA `user` a cui associare i metadati e lo salva nei dati dell'istruzione.
        let user_seeds: [&[u8]; 2] = [b"user", authority_pubkey.as_ref()];
        let (user_pubkey, _) = Pubkey::find_program_address(&user_seeds, &program_id);
        self.data.user_account.set_pubkey(user_pubkey);

        // Genera casualmente i quattro byte di PIN.
        self.data.pin1 = trident.gen_range(0..=u8::MAX);
        self.data.pin2 = trident.gen_range(0..=u8::MAX);
        self.data.pin3 = trident.gen_range(0..=u8::MAX);
        self.data.pin4 = trident.gen_range(0..=u8::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        // Recupera l'autorità, la marca come signer/writable e la finanzia per coprire la rent.
        let authority_pubkey = fuzz_accounts.authority.get_or_create(
            self.accounts.authority.account_id,
            trident,
            None,
            None,
        );
        self.accounts.authority.set_address(authority_pubkey);
        self.accounts.authority.set_is_signer();
        self.accounts.authority.set_is_writable();
        trident.airdrop(&authority_pubkey, 5 * LAMPORTS_PER_SOL);

        // Deriva il PDA `user_metadata` usando l'autorità come seed.
        let metadata_seeds: [&[u8]; 2] = [b"user_metadata", authority_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(&metadata_seeds, &program_id);
        self.accounts.user_metadata.set_address(metadata_pubkey);
        self.accounts.user_metadata.set_is_writable();

        // Reset dell'account per permettere più iterazioni di init senza conflitti.
        let default_account = AccountSharedData::new(0, 0, &solana_sdk::system_program::ID);
        trident
            .get_client()
            .set_account_custom(&metadata_pubkey, &default_account);

        // Salva il mapping nel pool per le istruzioni successive (es. read).
        fuzz_accounts.user_metadata.get_or_create(
            self.accounts.user_metadata.account_id,
            trident,
            Some(PdaSeeds::new(&metadata_seeds, program_id)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );

        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
    }
}
