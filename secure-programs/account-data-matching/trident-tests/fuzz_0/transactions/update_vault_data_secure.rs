// Builds the update_vault_data_secure transaction for the Account Data Matching fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
// Macro-generated transaction wrapper used by the fuzz harness.
#[derive(Debug, TridentTransaction, Default)]
pub struct UpdateVaultDataSecureTransaction {
    // Executes the secure update instruction within this transaction.
    pub instruction: UpdateVaultDataSecureInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for UpdateVaultDataSecureTransaction {
        // Share the deterministic account pool across transactions.
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // 1) Pubkey effettivamente usata come signer dall'istruzione di update
        //    Capture the signer key that the instruction actually consumed.
        let used_pk = self
            .instruction
            .accounts
            .vault_authority
            .pubkey()
            .to_bytes(); // [u8; 32]

        // 2) Leggi i bytes dell'account `vault` DOPO la transazione
        //    Snapshot the vault contents after execution to see who owns it.
        let vault_after = self
            .instruction
            .accounts
            .vault
            .get_snapshot_after()
            .data();

        // Struttura Anchor dell'account `Vault`:
        //    Layout reminder: discriminator (8) + owner (32) + data byte (1).
        // [8 bytes discriminator] + [32 bytes vault_authority] + [1 byte data]
        const DISC: usize = 8;
        const KEY: usize = 32;

        if vault_after.len() < DISC + KEY + 1 {
            return Err(FuzzingError::with_message(
                "Invariant: vault account data too short",
            ));
        }

        // 3) Owner atteso salvato nel conto (campo `vault_authority`)
        //    Read the recorded owner straight from the serialized data.
        let stored_owner_bytes = &vault_after[DISC..DISC + KEY];

        // 4) L'invariante: solo l'owner registrato nel vault può aggiornare
        //    Reject any transaction where signer ≠ stored owner to catch privilege issues.
        if used_pk != stored_owner_bytes {
            return Err(FuzzingError::with_message(
                "Invariant failed: non-owner updated the vault",
            ));
        }

        Ok(())
    }
}
