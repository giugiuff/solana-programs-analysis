// Sets up the secure_atomic_trade instruction for the Duplicate Mutable Accounts fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

const PROGRAM_ID: Pubkey = pubkey!("3CGZ6JWo2QENGahAopnv9Z5mwCHH6WgKKbLJk9y7zLu8");
const VAULT_ACCOUNT_SPACE: usize = 8 + 32 + 8;

#[derive(TridentInstruction, Default)]
#[program_id("3CGZ6JWo2QENGahAopnv9Z5mwCHH6WgKKbLJk9y7zLu8")]
#[discriminator([103u8, 142u8, 223u8, 71u8, 125u8, 105u8, 124u8, 51u8])]
pub struct SecureAtomicTradeInstruction {
    pub accounts: SecureAtomicTradeInstructionAccounts,
    pub data: SecureAtomicTradeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SecureAtomicTradeInstructionData)]
#[storage(FuzzAccounts)]
pub struct SecureAtomicTradeInstructionAccounts {
    #[account(signer)]
    pub signer_a: TridentAccount,

    #[account(signer)]
    pub signer_b: TridentAccount,

    #[account(mut)]
    pub vault_a: TridentAccount,

    #[account(mut)]
    pub vault_b: TridentAccount,

    #[account(mut)]
    pub fee_vault: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SecureAtomicTradeInstructionData {
    pub transfer_amount: u64,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for SecureAtomicTradeInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, _trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        self.data.transfer_amount = fuzz_accounts.atomic_trade.transfer_amount;
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let scenario = &fuzz_accounts.atomic_trade;

        let signer_a_pubkey = fuzz_accounts
            .signer_a
            .get_or_create(scenario.signer_a_id, trident, None, None);
        self.accounts.signer_a.set_address(signer_a_pubkey);
        self.accounts.signer_a.set_is_signer();
        self.accounts.signer_a.account_id = scenario.signer_a_id;

        let signer_b_pubkey = if scenario.duplicate_vaults {
            signer_a_pubkey
        } else {
            fuzz_accounts
                .signer_b
                .get_or_create(scenario.signer_b_id, trident, None, None)
        };
        self.accounts.signer_b.set_address(signer_b_pubkey);
        self.accounts.signer_b.set_is_signer();
        self.accounts.signer_b.account_id = if scenario.duplicate_vaults {
            scenario.signer_a_id
        } else {
            scenario.signer_b_id
        };

        let signer_a_bytes = signer_a_pubkey.to_bytes();
        let vault_a_seeds: [&[u8]; 2] = [b"vault", signer_a_bytes.as_ref()];

        let vault_a_pubkey = fuzz_accounts.vault_a.get_or_create(
            scenario.vault_a_id,
            trident,
            Some(PdaSeeds::new(&vault_a_seeds, PROGRAM_ID)),
            Some(AccountMetadata::new(
                10 * LAMPORTS_PER_SOL,
                VAULT_ACCOUNT_SPACE,
                PROGRAM_ID,
            )),
        );
        self.accounts.vault_a.set_address(vault_a_pubkey);
        self.accounts.vault_a.set_is_writable();
        self.accounts.vault_a.account_id = scenario.vault_a_id;

        let vault_b_pubkey = if scenario.duplicate_vaults {
            vault_a_pubkey
        } else {
            let signer_b_bytes = signer_b_pubkey.to_bytes();
            let vault_b_seeds: [&[u8]; 2] = [b"vault", signer_b_bytes.as_ref()];
            fuzz_accounts.vault_b.get_or_create(
                scenario.vault_b_id,
                trident,
                Some(PdaSeeds::new(&vault_b_seeds, PROGRAM_ID)),
                Some(AccountMetadata::new(
                    10 * LAMPORTS_PER_SOL,
                    VAULT_ACCOUNT_SPACE,
                    PROGRAM_ID,
                )),
            )
        };
        self.accounts.vault_b.set_address(vault_b_pubkey);
        self.accounts.vault_b.set_is_writable();
        self.accounts.vault_b.account_id = if scenario.duplicate_vaults {
            scenario.vault_a_id
        } else {
            scenario.vault_b_id
        };

        let fee_vault_seeds: [&[u8]; 1] = [b"fee_vault"];
        let fee_vault_pubkey = fuzz_accounts.fee_vault.get_or_create(
            scenario.fee_vault_id,
            trident,
            Some(PdaSeeds::new(&fee_vault_seeds, PROGRAM_ID)),
            Some(AccountMetadata::new(
                10 * LAMPORTS_PER_SOL,
                VAULT_ACCOUNT_SPACE,
                PROGRAM_ID,
            )),
        );
        self.accounts.fee_vault.set_address(fee_vault_pubkey);
        self.accounts.fee_vault.set_is_writable();
        self.accounts.fee_vault.account_id = scenario.fee_vault_id;
    }
}



