// Shared data types for the Signer Authorization Trident fuzz target, keeping instruction inputs consistent across cases.

use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
use trident_fuzz::types::AccountId;

pub const PROGRAM_ID: Pubkey = pubkey!("BDkpnjtGdVNhUVCY4iFcJFPy33j5hnPkf6cDHvsiBFCn");
pub const LEGIT_AUTHORITY_ID: AccountId = 0;
pub const ATTACKER_ID: AccountId = 1;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Escrow {
    pub authority: TridentPubkey,

    pub data: u8,
}
