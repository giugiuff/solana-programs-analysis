// Sets up the initialize_metadata instruction for the Revival Attack fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("oU4RYs2as9XzZ3Mn5DFn6hC6wNLcYr9VK11GnQPQbsm")]
#[discriminator([35u8, 215u8, 241u8, 156u8, 122u8, 208u8, 206u8, 212u8])]
pub struct InitializeMetadataInstruction {
    pub accounts: InitializeMetadataInstructionAccounts,
    pub data: InitializeMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeMetadataInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeMetadataInstructionAccounts {
    #[account(mut, signer)]
    pub creator: TridentAccount,

    #[account(mut)]
    pub metadata: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeMetadataInstructionData {
    pub secret1: u8,

    pub secret2: u8,

    pub secret3: u8,

    pub secret4: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeMetadataInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        self.data.secret1 = trident.gen_range(0..=u8::MAX);
        self.data.secret2 = trident.gen_range(0..=u8::MAX);
        self.data.secret3 = trident.gen_range(0..=u8::MAX);
        self.data.secret4 = trident.gen_range(0..=u8::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "oU4RYs2as9XzZ3Mn5DFn6hC6wNLcYr9VK11GnQPQbsm";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid revival attack program id");

        // Prepare the creator signer with sufficient balance to pay rent for the PDA.
        let creator_id = self.accounts.creator.account_id;
        let creator_pubkey = fuzz_accounts
            .creator
            .get_or_create(creator_id, trident, None, None);
        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();
        self.accounts.creator.set_is_writable();
        trident.airdrop(&creator_pubkey, 2 * LAMPORTS_PER_SOL);

        // Derive the metadata PDA and ensure it is pristine before initialization.
        let seeds: [&[u8]; 2] = [b"secret_metadata", creator_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(&seeds, &program_id);

        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();

        // Reset any previous state so that the init constraint can succeed across iterations.
        let default_account = AccountSharedData::new(0, 0, &solana_sdk::system_program::ID);
        trident
            .get_client()
            .set_account_custom(&metadata_pubkey, &default_account);

        // System program is fixed.
        self.accounts.system_program.set_address(solana_sdk::system_program::ID);

        // Persist PDA mapping in fuzz storage so it remains stable if reused later.
        fuzz_accounts.metadata.get_or_create(
            self.accounts.metadata.account_id,
            trident,
            Some(PdaSeeds::new(&seeds, program_id)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );
    }
}



