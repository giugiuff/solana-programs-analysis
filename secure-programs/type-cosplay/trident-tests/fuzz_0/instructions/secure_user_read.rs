// Sets up the secure_user_read instruction for the Type Cosplay fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618")]
#[discriminator([254u8, 146u8, 2u8, 189u8, 176u8, 106u8, 28u8, 66u8])]
pub struct SecureUserReadInstruction {
    pub accounts: SecureUserReadInstructionAccounts,
    pub data: SecureUserReadInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SecureUserReadInstructionData)]
#[storage(FuzzAccounts)]
pub struct SecureUserReadInstructionAccounts {
    pub user: TridentAccount,

    #[account(signer)]
    pub authority: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SecureUserReadInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for SecureUserReadInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        let authority_pubkey = fuzz_accounts
            .authority
            .get_or_create(self.accounts.authority.account_id, trident, None, None);
        self.accounts.authority.set_address(authority_pubkey);
        self.accounts.authority.set_is_signer();

        let user_account_size = 32 + 32 + 4;
        let fake_user_pubkey = fuzz_accounts.cosplay.get_or_create(
            self.accounts.user.account_id,
            trident,
            None,
            Some(AccountMetadata::new(
                5 * LAMPORTS_PER_SOL,
                user_account_size,
                program_id,
            )),
        );

        self.accounts.user.set_address(fake_user_pubkey);
        self.accounts.user.set_is_writable();
    }
}
