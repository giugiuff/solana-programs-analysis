// Defines lazily-created accounts so the PDA Privileges fuzzing scenario can reuse deterministic fixtures between instructions.

use crate::types::WithdrawScenario;
use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
pub struct FuzzAccounts {
    pub associated_token_program: AccountsStorage,

    pub token_program: AccountsStorage,

    pub vault: AccountsStorage,

    pub metadata_account: AccountsStorage,

    pub withdraw_destination: AccountsStorage,

    pub vault_creator: AccountsStorage,

    pub mint: AccountsStorage,

    pub system_program: AccountsStorage,

    pub creator: AccountsStorage,

    pub withdraw: WithdrawScenario,
}

impl Default for FuzzAccounts {
    fn default() -> Self {
        Self {
            associated_token_program: AccountsStorage::default(),
            token_program: AccountsStorage::default(),
            vault: AccountsStorage::default(),
            metadata_account: AccountsStorage::default(),
            withdraw_destination: AccountsStorage::default(),
            vault_creator: AccountsStorage::default(),
            mint: AccountsStorage::default(),
            system_program: AccountsStorage::default(),
            creator: AccountsStorage::default(),
            withdraw: WithdrawScenario::default(),
        }
    }
}
