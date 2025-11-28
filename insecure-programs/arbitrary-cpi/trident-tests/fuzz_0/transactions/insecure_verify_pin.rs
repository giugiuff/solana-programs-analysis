use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InsecureVerifyPinTransaction {
    pub instruction: InsecureVerifyPinInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/

/// Hook di Trident che consente di definire controlli e logica personalizzata attorno alla transazione.
impl TransactionHooks for InsecureVerifyPinTransaction {
    type IxAccounts = FuzzAccounts;

    /// Verifica che la transazione faccia CPI verso il programma previsto, bloccando deviazioni malevole.
    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // Programma legittimo che dovrebbe essere invocato dalla CPI.
        let expected_program =
            Pubkey::from_str("Ekq3FZqpHQ6coawYtyeG9QB3QWkx9zQGKSBewdQvUyyE").unwrap();
        // Programma effettivo configurato dall'istruzione.
        let actual_program = self.instruction.accounts.secret_program.pubkey();

        // Se il programma differisce da quello atteso, segnala un potenziale attacco CPI arbitrario.
        if actual_program != expected_program {
            return Err(FuzzingError::with_message(&format!(
                "Arbitrary CPI detected: expected program {}, got {}",
                expected_program, actual_program
            )));
        }

        Ok(())
    }
}
