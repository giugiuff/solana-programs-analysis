// Shared data types for the PDA Privileges Trident fuzz target, keeping instruction inputs consistent across cases.

use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
use trident_fuzz::types::AccountId;

pub const LEGIT_CREATOR_ID: AccountId = 0;
pub const ATTACKER_CREATOR_ID: AccountId = 1;
pub const VAULT_ACCOUNT_ID: AccountId = 0;
pub const METADATA_ACCOUNT_ID: AccountId = 0;
pub const MINT_ACCOUNT_ID: AccountId = 0;
pub const LEGIT_WITHDRAW_DEST_ID: AccountId = 0;
pub const ATTACKER_WITHDRAW_DEST_ID: AccountId = 1;

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct MetadataAccount {
    pub creator: TridentPubkey,
}

#[derive(Debug, Clone)]
pub struct WithdrawScenario {
    pub use_correct_creator: bool,
    pub legit_creator_id: AccountId,
    pub attacker_creator_id: AccountId,
    pub vault_account_id: AccountId,
    pub metadata_account_id: AccountId,
    pub mint_account_id: AccountId,
    pub legit_destination_id: AccountId,
    pub attacker_destination_id: AccountId,
}

impl Default for WithdrawScenario {
    fn default() -> Self {
        Self {
            use_correct_creator: true,
            legit_creator_id: LEGIT_CREATOR_ID,
            attacker_creator_id: ATTACKER_CREATOR_ID,
            vault_account_id: VAULT_ACCOUNT_ID,
            metadata_account_id: METADATA_ACCOUNT_ID,
            mint_account_id: MINT_ACCOUNT_ID,
            legit_destination_id: LEGIT_WITHDRAW_DEST_ID,
            attacker_destination_id: ATTACKER_WITHDRAW_DEST_ID,
        }
    }
}
