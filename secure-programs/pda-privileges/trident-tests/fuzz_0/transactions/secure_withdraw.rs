// Builds the secure_withdraw transaction for the PDA Privileges fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use crate::types::MetadataAccount;
use borsh::BorshDeserialize;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;
use trident_fuzz::fuzzing::solana_sdk::transaction::TransactionError;
use trident_fuzz::fuzzing::*;
use trident_fuzz::trident_accounts::SnapshotAccount;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct SecureWithdrawTransaction {
    pub instruction: SecureWithdrawInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for SecureWithdrawTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        const DISCRIMINATOR_LEN: usize = 8;

        let metadata_after = self
            .instruction
            .accounts
            .metadata_account
            .get_snapshot_after();
        let metadata_bytes = metadata_after.data();
        if metadata_bytes.len() < DISCRIMINATOR_LEN {
            return Err(FuzzingError::with_message(
                "Invariant failed: metadata account missing discriminator",
            ));
        }

        let metadata_account = MetadataAccount::try_from_slice(
            &metadata_bytes[DISCRIMINATOR_LEN..],
        )
        .map_err(|_| {
            FuzzingError::with_message("Invariant failed: unable to deserialize metadata account")
        })?;

        let expected_creator = metadata_account.creator.get_pubkey();
        let signer = self.instruction.accounts.creator.pubkey();

        if signer != expected_creator {
            return Err(FuzzingError::with_message(
                "Invariant failed: secure withdraw succeeded for non-creator",
            ));
        }

        let vault_before = parse_token_account(
            self.instruction.accounts.vault.get_snapshot_before(),
            "vault_before",
        )?;
        let vault_after = parse_token_account(
            self.instruction.accounts.vault.get_snapshot_after(),
            "vault_after",
        )?;
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

        let transferred = destination_after
            .amount
            .saturating_sub(destination_before.amount);
        let drained = vault_before.amount.saturating_sub(vault_after.amount);

        if transferred != drained {
            return Err(FuzzingError::with_message(&format!(
                "Invariant failed: transfer mismatch (transferred {transferred}, drained {drained})"
            )));
        }

        Ok(())
    }

    fn transaction_error_handler(&self, err: TransactionError) {
        if let TransactionError::InstructionError(_, _) = err {
            const DISCRIMINATOR_LEN: usize = 8;
            let metadata_before = self
                .instruction
                .accounts
                .metadata_account
                .get_snapshot_before();
            let metadata_bytes = metadata_before.data();

            if metadata_bytes.len() >= DISCRIMINATOR_LEN {
                if let Ok(metadata_account) =
                    MetadataAccount::try_from_slice(&metadata_bytes[DISCRIMINATOR_LEN..])
                {
                    let expected_creator = metadata_account.creator.get_pubkey();
                    if expected_creator != self.instruction.accounts.creator.pubkey() {
                        // Expected failure caused by mismatched creator; ignore.
                        return;
                    }
                }
            }
        }

        panic!("secure_withdraw failed unexpectedly: {err:?}");
    }
}

fn parse_token_account(
    account: &SnapshotAccount,
    label: &str,
) -> Result<SplTokenAccount, FuzzingError> {
    SplTokenAccount::unpack(account.data()).map_err(|_| {
        FuzzingError::with_message(&format!(
            "Invariant failed: unable to unpack {label} SPL token account"
        ))
    })
}
