// Defines lazily-created accounts so the Ownership Check fuzzing scenario can reuse deterministic fixtures between instructions.

use crate::types::OwnershipScenario;
use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
pub struct FuzzAccounts {
    pub token_account_owner: AccountsStorage,

    pub mint: AccountsStorage,

    pub token_account: AccountsStorage,

    pub ownership: OwnershipScenario,
}

impl Default for FuzzAccounts {
    fn default() -> Self {
        Self {
            token_account_owner: AccountsStorage::default(),
            mint: AccountsStorage::default(),
            token_account: AccountsStorage::default(),
            ownership: OwnershipScenario::default(),
        }
    }
}
