use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
use trident_fuzz::types::AccountId;

pub const LEGIT_SIGNER_ID: AccountId = 0;
pub const ATTACKER_SIGNER_ID: AccountId = 1;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct GlobalConfig {
    pub authority: TridentPubkey,

    pub additional_data: u8,
}

#[derive(Debug, Clone)]
pub struct InitializationScenario {
    pub use_correct_authority: bool,
    pub additional_data: u8,
    pub legit_signer_id: AccountId,
    pub attacker_signer_id: AccountId,
    pub global_config_id: AccountId,
    pub program_data_id: AccountId,
}

impl Default for InitializationScenario {
    fn default() -> Self {
        Self {
            use_correct_authority: true,
            additional_data: 0,
            legit_signer_id: LEGIT_SIGNER_ID,
            attacker_signer_id: ATTACKER_SIGNER_ID,
            global_config_id: 0,
            program_data_id: 0,
        }
    }
}
