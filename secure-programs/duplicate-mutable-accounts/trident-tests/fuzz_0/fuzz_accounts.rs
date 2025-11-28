// Defines lazily-created accounts so the Duplicate Mutable Accounts fuzzing scenario can reuse deterministic fixtures between instructions.

use crate::types::AtomicTradeScenario;
use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub authority: AccountsStorage,

    pub vault_a: AccountsStorage,

    pub system_program: AccountsStorage,

    pub signer_b: AccountsStorage,

    pub vault: AccountsStorage,

    pub vault_b: AccountsStorage,

    pub owner: AccountsStorage,

    pub signer_a: AccountsStorage,

    pub creator: AccountsStorage,

    pub fee_vault: AccountsStorage,

    pub atomic_trade: AtomicTradeScenario,
}
