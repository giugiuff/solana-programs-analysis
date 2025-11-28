use crate::fuzz_accounts::FuzzAccounts; // storage degli account generati nei test
use crate::types::*; // rappresentazione Borsh di User usata per i controlli
use borsh::{BorshDeserialize, BorshSerialize}; // codec dei parametri
use std::str::FromStr; // conversione del program id in Pubkey
use trident_fuzz::fuzzing::*; // API generiche Trident (Trident, PdaSeeds, ecc.)

#[derive(TridentInstruction, Default)]
#[program_id("5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618")]
#[discriminator([111u8, 17u8, 185u8, 250u8, 60u8, 122u8, 38u8, 254u8])]
pub struct InitializeUserInstruction {
    pub accounts: InitializeUserInstructionAccounts,
    pub data: InitializeUserInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeUserInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeUserInstructionAccounts {
    #[account(mut, signer)]
    pub authority: TridentAccount,

    #[account(mut)]
    pub user: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeUserInstructionData {
    pub metadata_account: TridentPubkey,

    pub age: u32,
}

impl InstructionHooks for InitializeUserInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        // Recupera l'autorità corrente per derivare PDAs coerenti.
        let authority_pubkey = fuzz_accounts.authority.get_or_create(
            self.accounts.authority.account_id,
            trident,
            None,
            None,
        );

        // Deriva il PDA `user_metadata` legato all'autorità e lo imposta nei dati dell'istruzione.
        let metadata_seeds: [&[u8]; 2] = [b"user_metadata", authority_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(&metadata_seeds, &program_id);

        self.data.metadata_account.set_pubkey(metadata_pubkey);
        // Sceglie un'età casuale (0-120) per riempire lo stato.
        self.data.age = trident.gen_range(0..=120u32);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        // Recupera l'autorità e la configura come signer+writable; airdrop per coprire la rent.
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

        // Calcola il PDA `user` deterministico per l'autorità.
        let user_seeds: [&[u8]; 2] = [b"user", authority_pubkey.as_ref()];
        let (user_pubkey, _) = Pubkey::find_program_address(&user_seeds, &program_id);
        self.accounts.user.set_address(user_pubkey);
        self.accounts.user.set_is_writable();

        // Azzera eventuali dati precedenti per consentire ripetute inizializzazioni nel fuzzer.
        let default_account = AccountSharedData::new(0, 0, &solana_sdk::system_program::ID);
        trident
            .get_client()
            .set_account_custom(&user_pubkey, &default_account);

        // Registra la mappatura nel pool di account per un riutilizzo coerente.
        fuzz_accounts.user.get_or_create(
            self.accounts.user.account_id,
            trident,
            Some(PdaSeeds::new(&user_seeds, program_id)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );

        // Aggiunge il programma di sistema richiesto da Anchor.
        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
    }
}
