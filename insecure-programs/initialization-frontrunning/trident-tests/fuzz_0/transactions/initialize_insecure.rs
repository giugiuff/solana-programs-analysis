use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use crate::types::{GlobalConfig, ATTACKER_SIGNER_ID};
use borsh::BorshDeserialize;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InitializeInsecureTransaction {
    pub instruction: InitializeInsecureInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
/// Hook di Trident che applica controlli specifici dopo l'esecuzione dell'istruzione di inizializzazione.
impl TransactionHooks for InitializeInsecureTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // Identifica quale account (legittimo o attaccante) ha firmato la transazione.
        let signer_id = self.instruction.accounts.signer.account_id;

        // Recupera lo stato finale dell'account `global_config` dopo l'istruzione.
        let global_config_snapshot = self.instruction.accounts.global_config.get_snapshot_after();

        // Controlla che i dati dell'account siano stati inizializzati (8 byte di discriminatore + payload).
        let data = global_config_snapshot.data();
        if data.len() <= 8 {
            return Err(FuzzingError::with_message(
                "global_config not initialized as expected",
            ));
        }

        // Deserializza la struct `GlobalConfig` saltando il discriminatore Anchor.
        let config = GlobalConfig::try_from_slice(&data[8..])
            .map_err(|_| FuzzingError::with_message("failed to deserialize global_config"))?;

        // Se l'attaccante è riuscito a settare se stesso come authority, segnala la vulnerabilità.
        if signer_id == ATTACKER_SIGNER_ID
            && config.authority.get_pubkey() == self.instruction.accounts.signer.pubkey()
        {
            return Err(FuzzingError::with_message(
                "initialize_insecure allowed unauthorized authority assignment",
            ));
        }

        Ok(())
    }
}
