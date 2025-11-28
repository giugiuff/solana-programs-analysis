use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct User {
    pub authority: TridentPubkey,

    pub metadata_account: TridentPubkey,

    pub age: u32,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct UserMetadata {
    pub authority: TridentPubkey,

    pub user_account: TridentPubkey,

    pub pin1: u8,

    pub pin2: u8,

    pub pin3: u8,

    pub pin4: u8,
}
