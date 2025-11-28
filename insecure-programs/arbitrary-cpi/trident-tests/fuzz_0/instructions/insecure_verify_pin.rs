use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("27KMmAJRGvicJx2BhBY8LvPhSTgVQVigDmzP9JCqYfoa")]
#[discriminator([47u8, 37u8, 197u8, 162u8, 198u8, 254u8, 68u8, 32u8])]
pub struct InsecureVerifyPinInstruction {
    pub accounts: InsecureVerifyPinInstructionAccounts,
    pub data: InsecureVerifyPinInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InsecureVerifyPinInstructionData)]
#[storage(FuzzAccounts)]
pub struct InsecureVerifyPinInstructionAccounts {
    #[account(signer)]
    pub author: TridentAccount,

    pub secret_information: TridentAccount,

    pub secret_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InsecureVerifyPinInstructionData {
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
/// Hook di Trident che definisce gli step preparatori dell'istruzione di verifica insicura.
impl InstructionHooks for InsecureVerifyPinInstruction {
    type IxAccounts = FuzzAccounts;

    /// Carica i byte del PIN nell'istruzione usando i valori generati durante il fuzzing.
    fn set_data(&mut self, _trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let pins = fuzz_accounts.pins;
        self.data.pin1 = pins[0];
        self.data.pin2 = pins[1];
        self.data.pin3 = pins[2];
        self.data.pin4 = pins[3];
    }

    /// Configura gli account richiesti, adattandosi alla modalità normale o d'attacco del fuzzer.
    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Programma atteso (legittimo) e programma malevolo usati nel contesto di fuzzing.
        let expected_program =
            Pubkey::from_str("Ekq3FZqpHQ6coawYtyeG9QB3QWkx9zQGKSBewdQvUyyE").unwrap();
        let hacked_program =
            Pubkey::from_str("6HHWk9zBVkq8XNTTpE9tidRjjfTSEkAQeTqi3hnB6xsW").unwrap();

        // Se `attack_mode` è attivo, usa l'account hacker; altrimenti quello dell'autore legittimo.
        // In parallelo seleziona l'ID del programma da invocare (legittimo o malevolo).
        let (author_storage, program_id) = if fuzz_accounts.attack_mode {
            (
                fuzz_accounts.hacker.get_or_create(1, trident, None, None),
                hacked_program,
            )
        } else {
            (
                fuzz_accounts.author.get_or_create(0, trident, None, None),
                expected_program,
            )
        };

        // Registra l'account dell'autore come firmatario (obbligatorio) senza mutarlo.
        self.accounts
            .author
            .set_account_meta(author_storage, true, false);
        // Imposta l'ID del programma target con cui verrà effettuata la CPI.
        self.accounts
            .secret_program
            .set_account_meta(program_id, false, false);

        // Recupera o calcola la PDA che ospita le informazioni segrete da verificare.
        let secret_pda = match fuzz_accounts.secret_pda {
            Some(pda) => pda,
            None => {
                let pda = Pubkey::find_program_address(
                    &[b"secret_info", author_storage.as_ref()],
                    &expected_program,
                )
                .0;
                // Memorizza il risultato per riutilizzarlo in istruzioni successive del fuzzer.
                fuzz_accounts.secret_pda = Some(pda);
                pda
            }
        };

        self.accounts
            .secret_information
            .set_account_meta(secret_pda, false, false);
    }
}
