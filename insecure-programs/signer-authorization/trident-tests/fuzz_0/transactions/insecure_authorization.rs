use crate::fuzz_accounts::FuzzAccounts; // contiene riferimenti a authority e attacker
use crate::instructions::*; // istruzione `insecure_authorization` generata dagli hook
use crate::types::Escrow; // versione Borsh dell'account escrow per i controlli
use borsh::BorshDeserialize; // decodifica dei dati dell'account
use trident_fuzz::fuzzing::*; // tipi e utility per i transaction hook

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InsecureAuthorizationTransaction {
    pub instruction: InsecureAuthorizationInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for InsecureAuthorizationTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // Chiave del firmatario effettivo (potrebbe essere l'attaccante).
        let signer_pubkey = self.instruction.accounts.authority.pubkey();

        // Snapshot dello stato dell'escrow dopo l'esecuzione dell'istruzione.
        let escrow_snapshot = self.instruction.accounts.escrow.get_snapshot_after();
        let escrow_state = Escrow::try_from_slice(escrow_snapshot.data_no_discriminator())
            .map_err(|_| {
                FuzzingError::with_message("Invariant failed: unable to decode escrow account")
            })?;

        let stored_authority = escrow_state.authority.get_pubkey();

        // Se l'autorità registrata non coincide con il firmatario, segnala la violazione.
        if stored_authority != signer_pubkey {
            return Err(FuzzingError::with_message(&format!(
                "Invariant failed: signer {} is not authorized for escrow {}",
                signer_pubkey, stored_authority,
            )));
        }

        Ok(())
    }
}
