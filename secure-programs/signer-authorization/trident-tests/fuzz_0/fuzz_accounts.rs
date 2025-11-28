// Defines lazily-created accounts so the Signer Authorization fuzzing scenario can reuse deterministic fixtures between instructions.

use trident_fuzz::fuzzing::solana_sdk::pubkey::Pubkey;
use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub authority: AccountsStorage,

    pub attacker: AccountsStorage,

    pub escrow: AccountsStorage,

    pub system_program: AccountsStorage,

    pub escrow_pda: Option<Pubkey>,

    pub escrow_initialized: bool,

    pub call_as_attacker: bool,
}
