use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use spl_token::solana_program::program_pack::Pack;
use spl_token::solana_program::pubkey::Pubkey as ProgramPubkey;
use spl_token::state::Account as SplTokenAccount;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InsecureLogBalanceV2Transaction {
    pub instruction: InsecureLogBalanceV2Instruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for InsecureLogBalanceV2Transaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        // L'handler dichiara questo owner: lo useremo come confronto.
        let declared_owner = self.instruction.accounts.token_account_owner.pubkey();
        let declared_owner = ProgramPubkey::new_from_array(declared_owner.to_bytes());

        let token_snapshot = self.instruction.accounts.token_account.get_snapshot_after();

        // Guardiamo lo stato SPL reale dell'account token appena letto.
        let token_state = SplTokenAccount::unpack(token_snapshot.data()).map_err(|_| {
            FuzzingError::with_message("Invariant failed: unable to unpack SPL token account")
        })?;

        if token_state.owner != declared_owner {
            return Err(FuzzingError::with_message(&format!(
                "Invariant failed: program accepted mismatched token ownership (actual owner: {actual}, declared owner: {declared})",
                actual = token_state.owner,
                declared = declared_owner,
            )));
        }

        Ok(())
    }
}