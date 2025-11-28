use crate::fuzz_accounts::FuzzAccounts; // Archivio degli account condivisi tra gli hook.
use crate::instructions::*; // Importa l'istruzione `InsecureWithdraw` e le relative API.
use crate::types::MetadataAccount; // Struct Anchor serializzata che descrive l'authority del vault.
use borsh::BorshDeserialize; // Necessaria per decodificare l'account Metadata dopo la transazione.
use spl_token::solana_program::program_pack::Pack; // Trait per (de)serializzare account SPL.
use spl_token::state::Account as SplTokenAccount; // Modello SPL dell'account token.
use trident_fuzz::fuzzing::*; // Macro, errori e tipi base del framework Trident.
use trident_fuzz::trident_accounts::SnapshotAccount; // Rappresenta lo stato degli account prima/dopo l'istruzione.

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InsecureWithdrawTransaction {
    pub instruction: InsecureWithdrawInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for InsecureWithdrawTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // Chiave pubblica che ha firmato la transazione (potrebbe essere l'attaccante).
        let attacker = self.instruction.accounts.creator.pubkey();

        // Read the metadata PDA to learn who the vault creator really is.
        let metadata_after = self
            .instruction
            .accounts
            .metadata_account
            .get_snapshot_after();
        let metadata_bytes = metadata_after.data();
        const DISCRIMINATOR_LEN: usize = 8;
        if metadata_bytes.len() < DISCRIMINATOR_LEN {
            return Err(FuzzingError::with_message(
                "Invariant failed: metadata account contains no data",
            ));
        }

        // Ricostruisce la struct Metadata per individuare il legittimo proprietario del vault.
        let metadata_account = MetadataAccount::try_from_slice(
            &metadata_bytes[DISCRIMINATOR_LEN..],
        )
        .map_err(|_| {
            FuzzingError::with_message("Invariant failed: unable to deserialize metadata account")
        })?;
        let expected_creator = metadata_account.creator.get_pubkey();

        if expected_creator == attacker {
            return Ok(());
        }

        // Analizza lo stato SPL prima/dopo della CPI per il vault della vittima.
        let vault_before = parse_token_account(
            self.instruction.accounts.vault.get_snapshot_before(),
            "vault_before",
        )?;
        let vault_after = parse_token_account(
            self.instruction.accounts.vault.get_snapshot_after(),
            "vault_after",
        )?;
        // ...e per l'ATA di destinazione controllata dall'attaccante.
        let destination_before = parse_token_account(
            self.instruction
                .accounts
                .withdraw_destination
                .get_snapshot_before(),
            "destination_before",
        )?;
        let destination_after = parse_token_account(
            self.instruction
                .accounts
                .withdraw_destination
                .get_snapshot_after(),
            "destination_after",
        )?;

        // Se il saldo dell'attaccante aumenta, segnala l'esfiltrazione non autorizzata.
        if destination_after.amount > destination_before.amount {
            return Err(FuzzingError::with_message(&format!(
                "Invariant failed: attacker {} drained {} tokens from vault owned by {}",
                attacker,
                destination_after
                    .amount
                    .saturating_sub(destination_before.amount),
                expected_creator
            )));
        }

        // Se il vault si svuota senza che il creator legittimo abbia firmato, è un bug.
        if vault_after.amount < vault_before.amount {
            return Err(FuzzingError::with_message(&format!(
                "Invariant failed: vault balance decreased from {} to {} without authorised creator",
                vault_before.amount, vault_after.amount
            )));
        }

        Ok(())
    }

    fn transaction_error_handler(&self, error: TransactionError) {
        if std::env::var("TRIDENT_FUZZ_VERBOSE").is_ok() {
            println!("Transaction failed: {:?}", error);
            // Fornisce dettagli diagnostici sugli owner degli account coinvolti.
            let vault_before = self.instruction.accounts.vault.get_snapshot_before();
            let metadata_before = self
                .instruction
                .accounts
                .metadata_account
                .get_snapshot_before();
            let destination_before = self
                .instruction
                .accounts
                .withdraw_destination
                .get_snapshot_before();
            let mint_before = self.instruction.accounts.mint.get_snapshot_before();
            let creator_before = self.instruction.accounts.creator.get_snapshot_before();
            println!(
                "Owners => vault: {}, destination: {}, metadata: {}, mint: {}, creator: {}",
                vault_before.owner(),
                destination_before.owner(),
                metadata_before.owner(),
                mint_before.owner(),
                creator_before.owner()
            );
        }
    }
}

fn parse_token_account(
    account: &SnapshotAccount,
    label: &str,
) -> Result<SplTokenAccount, FuzzingError> {
    // Converte un account SPL serializzato (prima/dopo) in struttura `SplTokenAccount`.
    SplTokenAccount::unpack(account.data()).map_err(|_| {
        FuzzingError::with_message(&format!(
            "Invariant failed: unable to unpack {label} SPL token account"
        ))
    })
}
