// Defines lazily-created accounts so the Arbitrary CPI fuzzing scenario can reuse deterministic fixtures between instructions.

use solana_sdk::pubkey::Pubkey;
use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub secret_information: AccountsStorage,

    pub author: AccountsStorage,

    pub system_program: AccountsStorage,

    pub secret_program: AccountsStorage,

    pub hacker: AccountsStorage,

    pub secret_pda: Option<Pubkey>,

    pub attack_mode: bool,

    pub pins: [u8; 4],
}
