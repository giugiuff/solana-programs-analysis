use crate::types::AtomicTradeScenario;
use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
pub struct FuzzAccounts {
    pub vault: AccountsStorage,

    pub owner: AccountsStorage,

    pub signer_a: AccountsStorage,

    pub authority: AccountsStorage,

    pub creator: AccountsStorage,

    pub vault_a: AccountsStorage,

    pub fee_vault: AccountsStorage,

    pub vault_b: AccountsStorage,

    pub system_program: AccountsStorage,

    pub signer_b: AccountsStorage,

    pub atomic_trade: AtomicTradeScenario,
}

impl Default for FuzzAccounts {
    fn default() -> Self {
        Self {
            vault: AccountsStorage::default(),
            owner: AccountsStorage::default(),
            signer_a: AccountsStorage::default(),
            authority: AccountsStorage::default(),
            creator: AccountsStorage::default(),
            vault_a: AccountsStorage::default(),
            fee_vault: AccountsStorage::default(),
            vault_b: AccountsStorage::default(),
            system_program: AccountsStorage::default(),
            signer_b: AccountsStorage::default(),
            atomic_trade: AtomicTradeScenario::default(),
        }
    }
}