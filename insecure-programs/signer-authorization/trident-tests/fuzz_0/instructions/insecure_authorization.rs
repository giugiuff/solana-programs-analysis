use crate::fuzz_accounts::FuzzAccounts; // storage con authority, attacker e PDA
use borsh::{BorshDeserialize, BorshSerialize}; // codec per l'input `data`
use solana_sdk::pubkey::Pubkey; // utile per calcolare/riusare il PDA `escrow`
use std::str::FromStr; // conversione del program id in Pubkey
use trident_fuzz::fuzzing::*; // API generiche di Trident

#[derive(TridentInstruction, Default)]
#[program_id("9e2aBh4MXpyHAxr2sq8guL4dVXiUA3CaJquQ4pnexQha")]
#[discriminator([40u8, 130u8, 183u8, 21u8, 0u8, 191u8, 10u8, 136u8])]
pub struct InsecureAuthorizationInstruction {
    pub accounts: InsecureAuthorizationInstructionAccounts,
    pub data: InsecureAuthorizationInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InsecureAuthorizationInstructionData)]
#[storage(FuzzAccounts)]
pub struct InsecureAuthorizationInstructionAccounts {
    #[account(
        signer,
        storage::name = authority,
        storage::account_id = 0,
    )]
    pub authority: TridentAccount,

    #[account(
        mut,
        storage::name = escrow,
        storage::account_id = 0,
    )]
    pub escrow: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InsecureAuthorizationInstructionData {
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
impl InstructionHooks for InsecureAuthorizationInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        // Valore casuale che finirà nel campo `escrow.data`.
        self.data.data = trident.gen_range(0..=u8::MAX);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        let program_id = Pubkey::from_str("9e2aBh4MXpyHAxr2sq8guL4dVXiUA3CaJquQ4pnexQha").unwrap();
        // Recupera il PDA `escrow` salvato durante l'inizializzazione, oppure lo calcola sul momento.
        let escrow_pda = match fuzz_accounts.escrow_pda {
            Some(pubkey) => pubkey,
            None => {
                let (pubkey, _) = Pubkey::find_program_address(&[b"escrow"], &program_id);
                fuzz_accounts.escrow_pda = Some(pubkey);
                pubkey
            }
        };

        // Ottiene sia l'autorità legittima sia un attaccante per simulare la mancanza di controlli.
        let legit_authority = fuzz_accounts
            .authority
            .get_or_create(0, trident, None, None);
        let rogue_authority = fuzz_accounts.attacker.get_or_create(1, trident, None, None);

        // Con probabilità 50% usa l'attaccante come signer, mostrando la vulnerabilità.
        let use_attacker = trident.gen_range(0..100) < 50;
        let chosen_signer = if use_attacker {
            rogue_authority
        } else {
            legit_authority
        };

        self.accounts
            .authority
            .set_account_meta(chosen_signer, true, true);

        // Fornisce il PDA come account mutabile, senza ulteriori controlli.
        self.accounts
            .escrow
            .set_account_meta(escrow_pda, false, true);
    }
}
