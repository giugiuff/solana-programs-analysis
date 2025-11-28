use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub withdraw_destination: AccountsStorage,

    pub metadata_account: AccountsStorage,

    pub system_program: AccountsStorage,

    pub mint: AccountsStorage,

    pub associated_token_program: AccountsStorage,

    pub vault: AccountsStorage,

    pub creator: AccountsStorage,

    pub vault_creator: AccountsStorage,

    pub token_program: AccountsStorage,
}
