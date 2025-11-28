// Sets up the secure_authorization instruction for the Signer Authorization fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("BDkpnjtGdVNhUVCY4iFcJFPy33j5hnPkf6cDHvsiBFCn")]
#[discriminator([48u8, 164u8, 130u8, 107u8, 92u8, 141u8, 75u8, 247u8])]
pub struct SecureAuthorizationInstruction {
    pub accounts: SecureAuthorizationInstructionAccounts,
    pub data: SecureAuthorizationInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SecureAuthorizationInstructionData)]
#[storage(FuzzAccounts)]
pub struct SecureAuthorizationInstructionAccounts {
    #[account(signer)]
    pub authority: TridentAccount,

    #[account(mut)]
    pub escrow: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SecureAuthorizationInstructionData {
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
impl InstructionHooks for SecureAuthorizationInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        self.data.data = trident.gen_range(0..=u8::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let program_id = PROGRAM_ID;
        let escrow_pda = match fuzz_accounts.escrow_pda {
            Some(pubkey) => pubkey,
            None => {
                let (pubkey, _) = Pubkey::find_program_address(&[b"escrow"], &program_id);
                fuzz_accounts.escrow_pda = Some(pubkey);
                pubkey
            }
        };

        let legitimate =
            fuzz_accounts
                .authority
                .get_or_create(LEGIT_AUTHORITY_ID, trident, None, None);
        let attacker = fuzz_accounts
            .attacker
            .get_or_create(ATTACKER_ID, trident, None, None);

        let chosen = if fuzz_accounts.call_as_attacker {
            attacker
        } else {
            legitimate
        };

        self.accounts.authority.set_account_meta(chosen, true, true);
        self.accounts.authority.account_id = if fuzz_accounts.call_as_attacker {
            ATTACKER_ID
        } else {
            LEGIT_AUTHORITY_ID
        };

        self.accounts
            .escrow
            .set_account_meta(escrow_pda, false, true);
    }
}
