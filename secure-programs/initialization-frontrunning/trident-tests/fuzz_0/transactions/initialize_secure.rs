// Builds the initialize_secure transaction for the Initialization Frontrunning fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InitializeSecureTransaction {
    pub instruction: InitializeSecureInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for InitializeSecureTransaction {
    type IxAccounts = FuzzAccounts;
}
