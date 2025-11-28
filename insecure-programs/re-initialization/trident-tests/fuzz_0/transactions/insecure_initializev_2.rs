use crate::fuzz_accounts::FuzzAccounts; // include flag attaccante/legittimo anche per la v2
use crate::instructions::*; // istruzione v2 che marca `is_initialized`
use crate::transactions::assert_metadata_creator_immutable; // controllo riutilizzabile sull'author
use trident_fuzz::fuzzing::*; // API principali di Trident

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InsecureInitializev2Transaction {
    pub instruction: InsecureInitializev2Instruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for InsecureInitializev2Transaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // Re-uso dell'invariante: il flag `is_initialized` non deve permettere cambio di ownership.
        assert_metadata_creator_immutable(
            &self.instruction.accounts.metadata,
            self.instruction.accounts.creator.account_id,
        )
    }
}
