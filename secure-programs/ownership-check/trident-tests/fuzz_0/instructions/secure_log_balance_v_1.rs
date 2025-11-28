// Sets up the secure_log_balance_v_1 instruction for the Ownership Check fuzz harness, wiring accounts and data so the PDA/authority checks are exercised.

use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::account::AccountSharedData;
use solana_sdk::rent::Rent;
use spl_token::solana_program::program_option::COption;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplMint};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("4u6h9QAMT8TuXauVZa9ieeQext18EtjecnX95xxw4xaa")]
#[discriminator([127u8, 28u8, 217u8, 244u8, 4u8, 93u8, 76u8, 81u8])]
pub struct SecureLogBalanceV1Instruction {
    pub accounts: SecureLogBalanceV1InstructionAccounts,
    pub data: SecureLogBalanceV1InstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(SecureLogBalanceV1InstructionData)]
#[storage(FuzzAccounts)]
pub struct SecureLogBalanceV1InstructionAccounts {
    pub mint: TridentAccount,

    pub token_account: TridentAccount,

    #[account(signer)]
    pub token_account_owner: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct SecureLogBalanceV1InstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for SecureLogBalanceV1Instruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let scenario = &fuzz_accounts.ownership;

        // Prepare the declared owner account.
        let legit_owner = fuzz_accounts.token_account_owner.get_or_create(
            scenario.legit_owner_id,
            trident,
            None,
            None,
        );
        let attacker_owner = fuzz_accounts.token_account_owner.get_or_create(
            scenario.attacker_owner_id,
            trident,
            None,
            None,
        );

        let declared_owner = if scenario.use_correct_owner {
            legit_owner
        } else {
            attacker_owner
        };

        self.accounts
            .token_account_owner
            .set_address(declared_owner);
        self.accounts.token_account_owner.set_is_signer();
        self.accounts.token_account_owner.account_id = if scenario.use_correct_owner {
            scenario.legit_owner_id
        } else {
            scenario.attacker_owner_id
        };

        // Create/refresh the mint account to look like a valid SPL mint.
        let mint_pubkey =
            fuzz_accounts
                .mint
                .get_or_create(scenario.mint_account_id, trident, None, None);
        let rent = Rent::default();
        let token_program = Pubkey::new_from_array(spl_token::id().to_bytes());
        let mut mint_account = AccountSharedData::new(
            rent.minimum_balance(SplMint::LEN),
            SplMint::LEN,
            &token_program,
        );

        let mint_authority = trident.gen_pubkey();
        let mint_authority_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(mint_authority.to_bytes());
        let decimals = trident.gen_range(0..=9);
        let mut mint_bytes = vec![0u8; SplMint::LEN];
        SplMint::pack(
            SplMint {
                is_initialized: true,
                mint_authority: COption::Some(mint_authority_spl),
                freeze_authority: COption::None,
                decimals,
                ..SplMint::default()
            },
            &mut mint_bytes,
        )
        .expect("pack mint");
        mint_account.set_data_from_slice(&mint_bytes);
        trident
            .get_client()
            .set_account_custom(&mint_pubkey, &mint_account);

        self.accounts.mint.set_address(mint_pubkey);
        self.accounts.mint.set_is_writable();
        self.accounts.mint.account_id = scenario.mint_account_id;

        // Create the SPL token account with potentially mismatching owner.
        let token_account_pubkey = fuzz_accounts.token_account.get_or_create(
            scenario.token_account_id,
            trident,
            None,
            None,
        );
        let mut token_account = AccountSharedData::new(
            rent.minimum_balance(SplTokenAccount::LEN),
            SplTokenAccount::LEN,
            &token_program,
        );

        // actual owner equals declared owner only when the scenario requests it.
        let actual_owner = if scenario.use_correct_owner {
            declared_owner
        } else {
            // generate a different owner to test the secure constraints.
            loop {
                let candidate = trident.gen_pubkey();
                if candidate != declared_owner {
                    break candidate;
                }
            }
        };

        let mint_pubkey_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(mint_pubkey.to_bytes());
        let actual_owner_spl =
            spl_token::solana_program::pubkey::Pubkey::new_from_array(actual_owner.to_bytes());

        let mut token_bytes = vec![0u8; SplTokenAccount::LEN];
        SplTokenAccount::pack(
            SplTokenAccount {
                mint: mint_pubkey_spl,
                owner: actual_owner_spl,
                amount: trident.gen_range(0..1_000_000_000u64),
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            &mut token_bytes,
        )
        .expect("pack token account");
        token_account.set_data_from_slice(&token_bytes);
        trident
            .get_client()
            .set_account_custom(&token_account_pubkey, &token_account);

        self.accounts
            .token_account
            .set_address(token_account_pubkey);
        self.accounts.token_account.set_is_writable();
        self.accounts.token_account.account_id = scenario.token_account_id;
    }
}
