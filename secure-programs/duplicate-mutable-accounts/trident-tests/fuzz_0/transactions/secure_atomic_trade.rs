// Builds the secure_atomic_trade transaction for the Duplicate Mutable Accounts fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;
use trident_fuzz::fuzzing::solana_sdk::transaction::TransactionError;
use trident_fuzz::fuzzing::processor::InstructionError;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct SecureAtomicTradeTransaction {
    pub instruction: SecureAtomicTradeInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for SecureAtomicTradeTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        if self
            .instruction
            .accounts
            .vault_a
            .pubkey()
            == self.instruction.accounts.vault_b.pubkey()
        {
            return Err(FuzzingError::with_message(
                "secure_atomic_trade should never execute with duplicated vault accounts",
            ));
        }

        Ok(())
    }

    fn transaction_error_handler(&self, err: TransactionError) {
        let vaults_equal = self
            .instruction
            .accounts
            .vault_a
            .pubkey()
            == self.instruction.accounts.vault_b.pubkey();

        if vaults_equal {
            match err {
                TransactionError::InstructionError(_, InstructionError::Custom(code)) => {
                    assert_eq!(code, 6000, "expected DuplicateVaults custom error (6000)");
                }
                other => panic!("expected DuplicateVaults custom error, got {other:?}"),
            }
        } else {
            panic!("secure_atomic_trade failed unexpectedly: {err:?}");
        }
    }
}
