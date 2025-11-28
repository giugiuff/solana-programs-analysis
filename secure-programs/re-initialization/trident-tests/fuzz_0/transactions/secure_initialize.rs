// Builds the secure_initialize transaction for the Re-Initialization fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use crate::types::ATTACKER_ID;
use trident_fuzz::fuzzing::solana_sdk::instruction::InstructionError;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct SecureInitializeTransaction {
    pub instruction: SecureInitializeInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for SecureInitializeTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_error_handler(&self, error: TransactionError) {
        if self.instruction.accounts.creator.account_id != ATTACKER_ID {
            panic!(
                "secure_initialize legitimate call unexpectedly failed: {:?}",
                error
            );
        }

        match error {
            TransactionError::InstructionError(_, InstructionError::ProgramFailedToComplete) => {}
            other => panic!(
                "secure_initialize attacker call should panic with ProgramFailedToComplete, got {:?}",
                other
            ),
        }
    }
}
