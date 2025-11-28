// Sets up the initialize_secret instruction for the Arbitrary CPI fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("67tPgGfjMJLMqH5u6h2Nf3pYfJn2qFPgxT8SYmKh3hzU")]
#[discriminator([180u8, 58u8, 164u8, 196u8, 254u8, 51u8, 113u8, 236u8])]
pub struct InitializeSecretInstruction {
    pub accounts: InitializeSecretInstructionAccounts,
    pub data: InitializeSecretInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeSecretInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeSecretInstructionAccounts {
    #[account(mut, signer)]
    pub author: TridentAccount,

    #[account(mut)]
    pub secret_information: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub secret_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeSecretInstructionData {
    pub pin1: u8,

    pub pin2: u8,

    pub pin3: u8,

    pub pin4: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeSecretInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let pins = [
            trident.gen_range(0..=u8::MAX),
            trident.gen_range(0..=u8::MAX),
            trident.gen_range(0..=u8::MAX),
            trident.gen_range(0..=u8::MAX),
        ];
        fuzz_accounts.pins = pins;

        self.data.pin1 = pins[0];
        self.data.pin2 = pins[1];
        self.data.pin3 = pins[2];
        self.data.pin4 = pins[3];
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let author = fuzz_accounts.author.get_or_create(0, trident, None, None);
        self.accounts.author.set_account_meta(author, true, true);
        trident.airdrop(&author, 5 * LAMPORTS_PER_SOL);

        let expected_program =
            Pubkey::from_str("CHSrnDkqAAzQzdZ3wuw5ifrafS6eNpLpcFsbcQfP8htj").unwrap();
        self.accounts
            .secret_program
            .set_account_meta(expected_program, false, false);

        self.accounts
            .system_program
            .set_account_meta(system_program::ID, false, false);

        let (secret_pda, _bump) =
            Pubkey::find_program_address(&[b"secret_info", author.as_ref()], &expected_program);
        self.accounts
            .secret_information
            .set_account_meta(secret_pda, false, true);

        fuzz_accounts.secret_pda = Some(secret_pda);
    }
}
