use crate::fuzz_accounts::FuzzAccounts; // storage condiviso per creator e metadata PDA
use borsh::{BorshDeserialize, BorshSerialize}; // codec per i segreti passati all'istruzione
use std::str::FromStr; // conversione del program id in Pubkey
use trident_fuzz::fuzzing::*; // API generiche di Trident

#[derive(TridentInstruction, Default)]
#[program_id("BxYhDihgJZZxUXqwoqvzbfD8G1fwNFKQyF8L5SEiNCQP")]
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
    // firmatario che dichiara il PIN da verificare
    pub creator: TridentAccount,

    #[account(mut)]
    // PDA dei metadati contenente i segreti da confrontare
    pub metadata: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
// valori del PIN da passare alla CPI di verifica
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

        // Impostiamo tutti i byte a zero per verificare l'esito post-chiusura (revival).
        self.data.secret1 = 0;
        self.data.secret2 = 0;
        self.data.secret3 = 0;
        self.data.secret4 = 0;
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "BxYhDihgJZZxUXqwoqvzbfD8G1fwNFKQyF8L5SEiNCQP";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid revival attack program id");

        // Recupera il firmatario deterministico e lo marca come signer.
        let creator_pubkey = fuzz_accounts.creator.get_or_create(
            self.accounts.creator.account_id,
            trident,
            None,
            None,
        );
        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();

        // Calcola il PDA dei metadati che potrebbe essere stato ripopolato dopo la chiusura.
        let seeds: [&[u8]; 2] = [b"secret_metadata", creator_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(&seeds, &program_id);

        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();

       
        // Aggiorna lo storage per riutilizzare la stessa PDA nelle invocazioni successive.
        fuzz_accounts.metadata.get_or_create(
            self.accounts.metadata.account_id,
            trident,
            Some(PdaSeeds::new(&seeds, program_id)),
            Some(AccountMetadata::new(0, 0, program_id)),
        );
    }
}
