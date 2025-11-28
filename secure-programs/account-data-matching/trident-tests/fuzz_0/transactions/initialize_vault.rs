// Builds the initialize_vault transaction for the Account Data Matching fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
// Macro generates the boilerplate to build and execute this transaction during fuzzing.
#[derive(Debug, TridentTransaction, Default)]
pub struct InitializeVaultTransaction {
    // Single-instruction transaction that seeds the vault state.
    pub instruction: InitializeVaultInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for InitializeVaultTransaction {
        // Share the same deterministic account cache across transactions.
    type IxAccounts = FuzzAccounts;
}
