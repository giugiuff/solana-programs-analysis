use crate::fuzz_accounts::FuzzAccounts; // storage che tiene traccia di authority e account "cosplay"
use borsh::{BorshDeserialize, BorshSerialize}; // codec per l'eventuale payload (qui vuoto)
use solana_sdk::pubkey::Pubkey; // per calcolare PDAs
use std::str::FromStr; // conversione dell'id del programma
use trident_fuzz::fuzzing::*; // API Trident per costruire account e transazioni

#[derive(TridentInstruction, Default)]
#[program_id("5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618")]
#[discriminator([35u8, 172u8, 22u8, 190u8, 26u8, 65u8, 99u8, 14u8])]
pub struct InsecureUserReadInstruction {
    pub accounts: InsecureUserReadInstructionAccounts,
    pub data: InsecureUserReadInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InsecureUserReadInstructionData)]
#[storage(FuzzAccounts)]
pub struct InsecureUserReadInstructionAccounts {
    pub user: TridentAccount,

    #[account(signer)]
    pub authority: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InsecureUserReadInstructionData {}

impl InstructionHooks for InsecureUserReadInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        // Usa sempre l'autorità legittima come signer dell'istruzione.
        let authority_pubkey = fuzz_accounts.authority.get_or_create(
            self.accounts.authority.account_id,
            trident,
            None,
            None,
        );
        self.accounts.authority.set_address(authority_pubkey);
        self.accounts.authority.set_is_signer();

        // Crea un account arbitrario con layout `User` senza impostarne il discriminator.
        // Questo riproduce il type cosplay: il programma on-chain lo interpreterà come `User`.
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
        // L'account deve essere mutabile perché l'istruzione legge e potenzialmente logga i dati.
        self.accounts.user.set_is_writable();
    }
}
