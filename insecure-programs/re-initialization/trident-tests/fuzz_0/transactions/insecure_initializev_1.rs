use crate::fuzz_accounts::FuzzAccounts; // stato condiviso che include flag attaccante/creatore
use crate::instructions::*; // definizione dell'istruzione v1 generata dagli hook
use crate::transactions::assert_metadata_creator_immutable; // helper che verifica che l'autore non cambi
use trident_fuzz::fuzzing::*; // macro e tipi base del framework Trident

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InsecureInitializev1Transaction {
    pub instruction: InsecureInitializev1Instruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for InsecureInitializev1Transaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // Valida che la re-inizializzazione non modifichi il campo `creator` dei metadati.
        assert_metadata_creator_immutable(
            &self.instruction.accounts.metadata,
            self.instruction.accounts.creator.account_id,
        )
    }
}
