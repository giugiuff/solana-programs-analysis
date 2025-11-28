// Defines lazily-created accounts so the Account Data Matching fuzzing scenario can reuse deterministic fixtures between instructions.

use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    // Fixed handle to the system program so Trident never randomizes it.
    pub system_program: AccountsStorage,

    // Deterministic pool of potential vault authorities used across flows.
    pub vault_authority: AccountsStorage,

    // PDA snapshots for the vault account so invariants can inspect before/after states.
    pub vault: AccountsStorage,
}
