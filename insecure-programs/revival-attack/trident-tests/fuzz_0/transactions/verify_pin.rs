use crate::fuzz_accounts::FuzzAccounts; // stato condiviso con flag e storage PDA
use crate::instructions::*; // istruzione `VerifyPin` generata dagli InstructionHooks
use trident_fuzz::fuzzing::*; // API per definire transaction hooks e client

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct VerifyPinTransaction {
    pub instruction: VerifyPinInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for VerifyPinTransaction {
    type IxAccounts = FuzzAccounts;

    fn pre_transaction(&self, client: &mut impl FuzzClient) {
        
        // Simula un revival: deposita lamport sul PDA dei metadati appena prima della verifica.
        let metadata_pubkey = self.instruction.accounts.metadata.pubkey();
        let mut metadata_account = client.get_account(&metadata_pubkey);
        let added = LAMPORTS_PER_SOL / 2;
        metadata_account.set_lamports(metadata_account.lamports().saturating_add(added));
        client.set_account_custom(&metadata_pubkey, &metadata_account);
    }

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        let before = self
            .instruction
            .accounts
            .metadata
            .get_snapshot_before()
            .lamports();
        let after = self
            .instruction
            .accounts
            .metadata
            .get_snapshot_after()
            .lamports();

        if before == 0 && after > 0 {
            // Se l'account era a zero ma dopo la CPI ha nuovamente lamport, abbiamo rilevato l'attacco.
            return Err(FuzzingError::with_message(
                "Revival attack detected: metadata PDA regained lamports after closure",
            ));
        }

        Ok(())
    }
}
