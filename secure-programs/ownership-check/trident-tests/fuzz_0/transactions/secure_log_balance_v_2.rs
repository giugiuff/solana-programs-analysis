// Builds the secure_log_balance_v_2 transaction for the Ownership Check fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use spl_token::solana_program::program_pack::Pack;
use spl_token::solana_program::pubkey::Pubkey as ProgramPubkey;
use spl_token::state::Account as SplTokenAccount;
use trident_fuzz::fuzzing::solana_sdk::transaction::TransactionError;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct SecureLogBalanceV2Transaction {
    pub instruction: SecureLogBalanceV2Instruction,
}

/// Methods for customizing transaction behavior:
/// - : Execute custom logic before transaction execution
/// - : Validate transaction-specific invariants
/// - : Custom handling of transaction errors
/// - : Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for SecureLogBalanceV2Transaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        let declared_owner = ProgramPubkey::new_from_array(
            self.instruction
                .accounts
                .token_account_owner
                .pubkey()
                .to_bytes(),
        );
        let token_snapshot = self.instruction.accounts.token_account.get_snapshot_after();

        let token_state = SplTokenAccount::unpack(token_snapshot.data()).map_err(|_| {
            FuzzingError::with_message("Invariant failed: unable to decode token account")
        })?;

        if token_state.owner != declared_owner {
            return Err(FuzzingError::with_message(
                "Invariant failed: secure handler accepted mismatched token ownership",
            ));
        }

        Ok(())
    }

    fn transaction_error_handler(&self, err: TransactionError) {
        if let TransactionError::InstructionError(_, _) = err {
            let declared_owner = ProgramPubkey::new_from_array(
                self.instruction
                    .accounts
                    .token_account_owner
                    .pubkey()
                    .to_bytes(),
            );
            let token_snapshot = self
                .instruction
                .accounts
                .token_account
                .get_snapshot_before();
            if let Ok(token_state) = SplTokenAccount::unpack(token_snapshot.data()) {
                if token_state.owner != declared_owner {
                    return;
                }
            }
        }

        panic!("secure_log_balance_v2 failed unexpectedly: {err:?}");
    }
}
