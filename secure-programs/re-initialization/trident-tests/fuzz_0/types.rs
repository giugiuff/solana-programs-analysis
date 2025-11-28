// Shared data types for the Re-Initialization Trident fuzz target, keeping instruction inputs consistent across cases.

use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
use trident_fuzz::types::AccountId;

pub const PROGRAM_ID: Pubkey = pubkey!("5wrTsnYf52JRAd8tSZDCCacQpFfLfVD64W7cHJ8FG7Ac");
pub const LEGIT_CREATOR_ID: AccountId = 1;
pub const ATTACKER_ID: AccountId = 2;
pub const METADATA_ID: AccountId = 1;
pub const INITIAL_LAMPORTS: u64 = 5 * LAMPORTS_PER_SOL;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeParameters {
    pub name: String,

    pub symbol: String,

    pub uri: String,

    pub year_of_creation: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Metadata {
    pub is_initialized: bool,

    pub creator: TridentPubkey,

    pub name: String,

    pub symbol: String,

    pub uri: String,

    pub year_of_creation: u64,
}
