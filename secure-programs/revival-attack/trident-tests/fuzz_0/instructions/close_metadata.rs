// Sets up the close_metadata instruction for the Revival Attack fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("oU4RYs2as9XzZ3Mn5DFn6hC6wNLcYr9VK11GnQPQbsm")]
#[discriminator([10u8, 220u8, 196u8, 138u8, 19u8, 60u8, 204u8, 130u8])]
pub struct CloseMetadataInstruction {
    pub accounts: CloseMetadataInstructionAccounts,
    pub data: CloseMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CloseMetadataInstructionData)]
#[storage(FuzzAccounts)]
pub struct CloseMetadataInstructionAccounts {
    #[account(mut, signer)]
    pub creator: TridentAccount,

    #[account(mut)]
    pub metadata: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct CloseMetadataInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CloseMetadataInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "oU4RYs2as9XzZ3Mn5DFn6hC6wNLcYr9VK11GnQPQbsm";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid revival attack program id");

        let creator_pubkey = fuzz_accounts
            .creator
            .get_or_create(self.accounts.creator.account_id, trident, None, None);
        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();
        self.accounts.creator.set_is_writable();

        let seeds: [&[u8]; 2] = [b"secret_metadata", creator_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(&seeds, &program_id);

        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();

        fuzz_accounts.metadata.get_or_create(
            self.accounts.metadata.account_id,
            trident,
            Some(PdaSeeds::new(&seeds, program_id)),
            Some(AccountMetadata::new(0, 0, program_id)),
        );
    }
}


