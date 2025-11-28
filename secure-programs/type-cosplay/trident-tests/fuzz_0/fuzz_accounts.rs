// Defines lazily-created accounts so the Type Cosplay fuzzing scenario can reuse deterministic fixtures between instructions.

use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub user: AccountsStorage,

    pub authority: AccountsStorage,

    pub user_metadata: AccountsStorage,

    pub system_program: AccountsStorage,

    pub cosplay: AccountsStorage,
}
