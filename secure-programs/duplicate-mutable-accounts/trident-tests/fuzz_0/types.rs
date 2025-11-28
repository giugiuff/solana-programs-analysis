// Shared data types for the Duplicate Mutable Accounts Trident fuzz target, keeping instruction inputs consistent across cases.

use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
use trident_fuzz::types::AccountId;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Vault {
    pub owner: TridentPubkey,

    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct AtomicTradeScenario {
    pub duplicate_vaults: bool,
    pub transfer_amount: u64,
    pub vault_a_balance: u64,
    pub vault_b_balance: u64,
    pub fee_vault_balance: u64,
    pub signer_a_id: AccountId,
    pub signer_b_id: AccountId,
    pub fee_authority_id: AccountId,
    pub vault_a_id: AccountId,
    pub vault_b_id: AccountId,
    pub fee_vault_id: AccountId,
}

impl Default for AtomicTradeScenario {
    fn default() -> Self {
        Self {
            duplicate_vaults: false,
            transfer_amount: 0,
            vault_a_balance: 0,
            vault_b_balance: 0,
            fee_vault_balance: 0,
            signer_a_id: 0,
            signer_b_id: 1,
            fee_authority_id: 2,
            vault_a_id: 0,
            vault_b_id: 1,
            fee_vault_id: 0,
        }
    }
}
