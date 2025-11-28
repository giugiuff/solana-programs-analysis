// Sets up the initialize instruction for the Signer Authorization fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("BDkpnjtGdVNhUVCY4iFcJFPy33j5hnPkf6cDHvsiBFCn")]
#[discriminator([175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeInstructionAccounts {
    #[account(mut, signer)]
    pub authority: TridentAccount,

    #[account(mut)]
    pub escrow: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeInstructionData {
    pub data: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let authority_pubkey =
            fuzz_accounts
                .authority
                .get_or_create(LEGIT_AUTHORITY_ID, trident, None, None);
        self.accounts
            .authority
            .set_account_meta(authority_pubkey, true, true);

        let (escrow_pda, _) = Pubkey::find_program_address(&[b"escrow"], &PROGRAM_ID);
        fuzz_accounts.escrow_pda = Some(escrow_pda);
        self.accounts
            .escrow
            .set_account_meta(escrow_pda, false, true);

        self.accounts.system_program.set_account_meta(
            solana_sdk::system_program::id(),
            false,
            false,
        );
    }
}
