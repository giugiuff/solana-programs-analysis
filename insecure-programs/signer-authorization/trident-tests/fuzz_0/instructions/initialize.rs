use crate::fuzz_accounts::FuzzAccounts; // storage condiviso di account e PDA usato nei test
use borsh::{BorshDeserialize, BorshSerialize}; // codec per serializzare l'input dato all'istruzione
use solana_sdk::{pubkey::Pubkey, system_program}; // utilità per calcolare PDAs e referenziare system program
use std::str::FromStr; // conversione del program id in Pubkey
use trident_fuzz::fuzzing::*; // API principali del framework Trident

#[derive(TridentInstruction, Default)]
#[program_id("9e2aBh4MXpyHAxr2sq8guL4dVXiUA3CaJquQ4pnexQha")]
#[discriminator([175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeInstructionAccounts {
    #[account(
        mut,
        signer,
        storage::name = authority,
        storage::account_id = 0,
    )]
    pub authority: TridentAccount,

    #[account(
        mut,
        storage::name = escrow,
        storage::account_id = 0,
    )]
    pub escrow: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeInstructionData {
    pub data: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        // Genera un valore casuale da salvare nello stato dell'escrow.
        self.data.data = trident.gen_range(0..=u8::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Recupera (o crea) l'autorità legittima che paga e firma l'inizializzazione.
        let authority_key = fuzz_accounts
            .authority
            .get_or_create(0, trident, None, None);

        self.accounts
            .authority
            .set_account_meta(authority_key, true, true);

        // Calcola il PDA `escrow` deterministico e lo salva nello storage condiviso.
        let program_id = Pubkey::from_str("9e2aBh4MXpyHAxr2sq8guL4dVXiUA3CaJquQ4pnexQha").unwrap();
        let (escrow_pda, _) = Pubkey::find_program_address(&[b"escrow"], &program_id);
        fuzz_accounts.escrow_pda = Some(escrow_pda);

        self.accounts
            .escrow
            .set_account_meta(escrow_pda, false, true);

        // Aggiunge l'account del system program richiesto da Anchor::init.
        self.accounts
            .system_program
            .set_account_meta(system_program::id(), false, false);
    }
}
