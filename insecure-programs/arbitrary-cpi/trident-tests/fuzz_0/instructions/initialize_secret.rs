use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("27KMmAJRGvicJx2BhBY8LvPhSTgVQVigDmzP9JCqYfoa")]
#[discriminator([180u8, 58u8, 164u8, 196u8, 254u8, 51u8, 113u8, 236u8])]
pub struct InitializeSecretInstruction {
    pub accounts: InitializeSecretInstructionAccounts,
    pub data: InitializeSecretInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeSecretInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeSecretInstructionAccounts {
    #[account(mut, signer)]
    pub author: TridentAccount,

    #[account(mut)]
    pub secret_information: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub secret_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeSecretInstructionData {
    pub pin1: u8,

    pub pin2: u8,

    pub pin3: u8,

    pub pin4: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/

/// Hook di Trident che definisce come l'istruzione viene inizializzata durante il fuzzing.
impl InstructionHooks for InitializeSecretInstruction {
    type IxAccounts = FuzzAccounts;

    /// Popola i quattro byte del PIN utilizzando i valori generati dal fuzzer.
    fn set_data(&mut self, _trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let pins = fuzz_accounts.pins;
        self.data.pin1 = pins[0];
        self.data.pin2 = pins[1];
        self.data.pin3 = pins[2];
        self.data.pin4 = pins[3];
    }

    /// Prepara gli account richiesti dall'istruzione `initialize_secret` prima di invocarla nel test.
    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Recupera o crea il portafoglio dell'autore in base all'indice 0, marcandolo come mutabile e firmatario.
        let author = fuzz_accounts.author.get_or_create(0, trident, None, None);
        self.accounts.author.set_account_meta(author, true, true);

        // Verifica che il CPI di destinazione corrisponda al programma atteso `arbitrary_cpi_expected`.
        let expected_program =
            Pubkey::from_str("Ekq3FZqpHQ6coawYtyeG9QB3QWkx9zQGKSBewdQvUyyE").unwrap();
        self.accounts
            .secret_program
            .set_account_meta(expected_program, false, false);

        // Imposta il programma di sistema, necessario per la creazione di nuovi account.
        self.accounts
            .system_program
            .set_account_meta(system_program::ID, false, false);

        // Calcola il PDA dove verrà memorizzato il PIN e lo registra come account mutabile.
        let (secret_pda, _bump) =
            Pubkey::find_program_address(&[b"secret_info", author.as_ref()], &expected_program);
        self.accounts
            .secret_information
            .set_account_meta(secret_pda, false, true);
        // Memorizza il PDA per riutilizzarlo nelle istruzioni successive del fuzzer.
        fuzz_accounts.secret_pda = Some(secret_pda);
    }
}
