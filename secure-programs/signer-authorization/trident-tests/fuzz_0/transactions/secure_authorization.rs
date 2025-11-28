// Builds the secure_authorization transaction for the Signer Authorization fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use crate::types::{Escrow, ATTACKER_ID};
use borsh::BorshDeserialize;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct SecureAuthorizationTransaction {
    pub instruction: SecureAuthorizationInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for SecureAuthorizationTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        let escrow_snapshot = self.instruction.accounts.escrow.get_snapshot_after();
        let escrow_state = Escrow::try_from_slice(escrow_snapshot.data_no_discriminator())
            .map_err(|_| {
                FuzzingError::with_message("Invariant failed: unable to decode escrow account")
            })?;

        let stored_authority = escrow_state.authority.get_pubkey();
        let signer = self.instruction.accounts.authority.pubkey();

        if stored_authority != signer {
            return Err(FuzzingError::with_message(&format!(
                "Invariant failed: signer {signer} is not authorized for escrow {stored}",
                stored = stored_authority
            )));
        }

        Ok(())
    }

    fn transaction_error_handler(&self, error: TransactionError) {
        if self.instruction.accounts.authority.account_id == ATTACKER_ID {
            return;
        }

        panic!("secure_authorization legitimate call unexpectedly failed: {error:?}");
    }
}
