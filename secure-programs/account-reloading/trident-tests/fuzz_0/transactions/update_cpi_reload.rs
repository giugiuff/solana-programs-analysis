// Builds the update_cpi_reload transaction for the Account Reloading fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use crate::types::Metadata;
use borsh::BorshDeserialize;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct UpdateCpiReloadTransaction {
    pub instruction: UpdateCpiReloadInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for UpdateCpiReloadTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        let before_snapshot = self.instruction.accounts.metadata.get_snapshot_before();
        let after_snapshot = self.instruction.accounts.metadata.get_snapshot_after();

        let before_data = before_snapshot.data();
        let after_data = after_snapshot.data();

        if before_data.len() <= 8 || after_data.len() <= 8 {
            return Err(FuzzingError::with_message(
                "Invariant failed: metadata account data too short",
            ));
        }

        let before_account = Metadata::try_from_slice(&before_data[8..]).map_err(|_| {
            FuzzingError::with_message("Invariant failed: unable to decode metadata before CPI")
        })?;
        let after_account = Metadata::try_from_slice(&after_data[8..]).map_err(|_| {
            FuzzingError::with_message("Invariant failed: unable to decode metadata after CPI")
        })?;

        if after_account.input != self.instruction.data.new_input {
            return Err(FuzzingError::with_message(&format!(
                "Reloaded flow did not persist new input ({}) onto account ({})",
                self.instruction.data.new_input, after_account.input
            )));
        }

        if before_account.input == after_account.input {
            return Err(FuzzingError::with_message(
                "Reloaded flow expected metadata to change but it did not",
            ));
        }

        Ok(())
    }
}
