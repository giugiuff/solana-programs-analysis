// Shared data types for the Ownership Check Trident fuzz target, keeping instruction inputs consistent across cases.

use trident_fuzz::types::AccountId;

pub const LEGIT_OWNER_ID: AccountId = 0;
pub const ATTACKER_OWNER_ID: AccountId = 1;
pub const MINT_ACCOUNT_ID: AccountId = 0;
pub const TOKEN_ACCOUNT_ID: AccountId = 0;

/// Scenario describing whether the provided owner matches the SPL token owner.
#[derive(Debug, Clone)]
pub struct OwnershipScenario {
    pub use_correct_owner: bool,
    pub legit_owner_id: AccountId,
    pub attacker_owner_id: AccountId,
    pub mint_account_id: AccountId,
    pub token_account_id: AccountId,
}

impl Default for OwnershipScenario {
    fn default() -> Self {
        Self {
            use_correct_owner: true,
            legit_owner_id: LEGIT_OWNER_ID,
            attacker_owner_id: ATTACKER_OWNER_ID,
            mint_account_id: MINT_ACCOUNT_ID,
            token_account_id: TOKEN_ACCOUNT_ID,
        }
    }
}
