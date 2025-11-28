// Shared data types for the Account Data Matching Trident fuzz target, keeping instruction inputs consistent across cases.

use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Vault {
    // Stored authority pubkey packed into the on-chain Vault account.
    pub vault_authority: TridentPubkey,

    // Single byte of payload that fuzz cases try to protect.
    pub data: u8,
}
