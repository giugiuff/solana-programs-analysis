use crate::types::InitializationScenario;
use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
pub struct FuzzAccounts {
    pub global_config: AccountsStorage,

    pub program_data: AccountsStorage,

    pub system_program: AccountsStorage,

    pub signer: AccountsStorage,

    pub initialization: InitializationScenario,
}

impl Default for FuzzAccounts {
    fn default() -> Self {
        Self {
            global_config: AccountsStorage::default(),
            program_data: AccountsStorage::default(),
            system_program: AccountsStorage::default(),
            signer: AccountsStorage::default(),
            initialization: InitializationScenario::default(),
        }
    }
}
