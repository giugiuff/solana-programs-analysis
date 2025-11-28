use trident_fuzz::fuzzing::solana_sdk::pubkey::Pubkey;
use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub system_program: AccountsStorage,

    pub escrow: AccountsStorage,

    pub authority: AccountsStorage,

    pub attacker: AccountsStorage,

    pub escrow_pda: Option<Pubkey>,

    pub escrow_initialized: bool,
}
