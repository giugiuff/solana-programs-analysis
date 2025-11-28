// Sets up the secure_initialize instruction for the Re-Initialization fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("5wrTsnYf52JRAd8tSZDCCacQpFfLfVD64W7cHJ8FG7Ac")]
#[discriminator([190u8, 34u8, 173u8, 35u8, 228u8, 9u8, 15u8, 124u8])]
pub struct SecureInitializeInstruction {
    pub accounts: SecureInitializeInstructionAccounts,
    pub data: SecureInitializeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SecureInitializeInstructionData)]
#[storage(FuzzAccounts)]
pub struct SecureInitializeInstructionAccounts {
    #[account(mut, signer)]
    pub creator: TridentAccount,

    #[account(mut)]
    pub metadata: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SecureInitializeInstructionData {
    pub parameters: InitializeParameters,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for SecureInitializeInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        self.data.parameters.name = trident.gen_string(1);
        self.data.parameters.symbol = trident.gen_string(1);
        self.data.parameters.uri = trident.gen_string(1);
        self.data.parameters.year_of_creation = trident.gen_range(1900_u64..=2100_u64);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let (creator_pubkey, creator_id) = if fuzz_accounts.call_as_attacker {
            (
                fuzz_accounts
                    .attacker
                    .get_or_create(ATTACKER_ID, trident, None, None),
                ATTACKER_ID,
            )
        } else {
            (
                fuzz_accounts
                    .creator
                    .get_or_create(LEGIT_CREATOR_ID, trident, None, None),
                LEGIT_CREATOR_ID,
            )
        };

        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();
        self.accounts.creator.account_id = creator_id;

        let metadata_pubkey = fuzz_accounts.metadata.get_or_create(
            METADATA_ID,
            trident,
            Some(PdaSeeds::new(&[b"metadata"], PROGRAM_ID)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );
        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();
        self.accounts.metadata.account_id = METADATA_ID;

        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
    }
}
