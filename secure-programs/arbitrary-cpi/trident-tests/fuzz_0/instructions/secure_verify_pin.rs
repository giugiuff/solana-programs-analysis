// Sets up the secure_verify_pin instruction for the Arbitrary CPI fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("67tPgGfjMJLMqH5u6h2Nf3pYfJn2qFPgxT8SYmKh3hzU")]
#[discriminator([213u8, 253u8, 170u8, 107u8, 230u8, 101u8, 102u8, 106u8])]
pub struct SecureVerifyPinInstruction {
    pub accounts: SecureVerifyPinInstructionAccounts,
    pub data: SecureVerifyPinInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SecureVerifyPinInstructionData)]
#[storage(FuzzAccounts)]
pub struct SecureVerifyPinInstructionAccounts {
    #[account(signer)]
    pub author: TridentAccount,

    pub secret_information: TridentAccount,

    #[account(address = "CHSrnDkqAAzQzdZ3wuw5ifrafS6eNpLpcFsbcQfP8htj")]
    pub secret_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SecureVerifyPinInstructionData {
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
impl InstructionHooks for SecureVerifyPinInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, _trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let pins = fuzz_accounts.pins;
        self.data.pin1 = pins[0];
        self.data.pin2 = pins[1];
        self.data.pin3 = pins[2];
        self.data.pin4 = pins[3];
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let author = fuzz_accounts.author.get_or_create(0, trident, None, None);
        self.accounts.author.set_account_meta(author, true, false);

        let expected_program =
            Pubkey::from_str("CHSrnDkqAAzQzdZ3wuw5ifrafS6eNpLpcFsbcQfP8htj").unwrap();
        let hacked_program =
            Pubkey::from_str("D3kqNE7AxMUYLbzZ8NyNPjgx6fbbsTeJ6GSXZtLeE5cC").unwrap();

        let (secret_pda, _bump) = Pubkey::find_program_address(
            &[b"secret_info", author.as_ref()],
            &expected_program,
        );
        fuzz_accounts.secret_pda = Some(secret_pda);

        let program_id = if fuzz_accounts.attack_mode {
            hacked_program
        } else {
            expected_program
        };
        self.accounts
            .secret_program
            .set_account_meta(program_id, false, false);

        self.accounts
            .secret_information
            .set_account_meta(secret_pda, false, false);
    }
}
