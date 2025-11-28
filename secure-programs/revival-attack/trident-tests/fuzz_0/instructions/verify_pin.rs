// Sets up the verify_pin instruction for the Revival Attack fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("oU4RYs2as9XzZ3Mn5DFn6hC6wNLcYr9VK11GnQPQbsm")]
#[discriminator([95u8, 197u8, 159u8, 142u8, 189u8, 29u8, 159u8, 21u8])]
pub struct VerifyPinInstruction {
    pub accounts: VerifyPinInstructionAccounts,
    pub data: VerifyPinInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(VerifyPinInstructionData)]
#[storage(FuzzAccounts)]
pub struct VerifyPinInstructionAccounts {
    #[account(signer)]
    pub creator: TridentAccount,

    #[account(mut)]
    pub metadata: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct VerifyPinInstructionData {
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
impl InstructionHooks for VerifyPinInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, _trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        // After the close handler runs the metadata secrets are zeroed out.
        self.data.secret1 = 0;
        self.data.secret2 = 0;
        self.data.secret3 = 0;
        self.data.secret4 = 0;
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "oU4RYs2as9XzZ3Mn5DFn6hC6wNLcYr9VK11GnQPQbsm";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid revival attack program id");

        let creator_pubkey = fuzz_accounts
            .creator
            .get_or_create(self.accounts.creator.account_id, trident, None, None);
        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();

        let seeds: [&[u8]; 2] = [b"secret_metadata", creator_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(&seeds, &program_id);

        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();

        // Ensure the fuzz storage tracks the PDA so subsequent instructions reuse it.
        fuzz_accounts.metadata.get_or_create(
            self.accounts.metadata.account_id,
            trident,
            Some(PdaSeeds::new(&seeds, program_id)),
            Some(AccountMetadata::new(0, 0, program_id)),
        );
    }
}



