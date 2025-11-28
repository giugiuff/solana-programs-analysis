use crate::fuzz_accounts::FuzzAccounts; // archivio degli account condivisi tra gli hook di fuzzing
use crate::types::*; // costanti e tipi (PROGRAM_ID, ATTACKER_ID, InitializeParameters, ecc.)
use borsh::{BorshDeserialize, BorshSerialize}; // serializzazione usata per l'input dell'istruzione
use trident_fuzz::fuzzing::*; // macro e API principali del framework Trident

#[derive(TridentInstruction, Default)]
#[program_id("BmeJbj9adPfVGT3S8JJ7uMWkDgmC9xfJPijHWrWii9Nn")]
#[discriminator([229u8, 87u8, 212u8, 190u8, 215u8, 222u8, 119u8, 249u8])]
pub struct InsecureInitializev1Instruction {
    pub accounts: InsecureInitializev1InstructionAccounts,
    pub data: InsecureInitializev1InstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InsecureInitializev1InstructionData)]
#[storage(FuzzAccounts)]
// descrive gli account richiesti dall'istruzione Anchor e come vengono prelevati dallo storage
pub struct InsecureInitializev1InstructionAccounts {
    #[account(mut, signer)]
    // firmatario che invoca l'inizializzazione (può essere attaccante o creatore legittimo)
    pub creator: TridentAccount,

    #[account(mut)]
    // PDA dei metadati soggetto a re-initialization
    pub metadata: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    // programma di sistema necessario per creare/finanziare l'account
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
// nessun campo extra: la struct incapsula i parametri dell'istruzione Anchor
pub struct InsecureInitializev1InstructionData {
    pub parameters: InitializeParameters,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InsecureInitializev1Instruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        // genera parametri testuali casuali per simulare dati sovrascrivibili
        self.data.parameters.name = trident.gen_string(1);
        self.data.parameters.symbol = trident.gen_string(1);
        self.data.parameters.uri = trident.gen_string(1);
        self.data.parameters.year_of_creation = trident.gen_range(1900_u64..=2100_u64);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // seleziona il firmatario: 50% attaccante, 50% creatore genuino
        let (creator_pubkey, creator_id) = if fuzz_accounts.call_as_attacker {
            (
                fuzz_accounts
                    .attacker
                    .get_or_create(ATTACKER_ID, trident, None, None),
                ATTACKER_ID,
            )
        } else {
            (
                fuzz_accounts
                    .creator
                    .get_or_create(LEGIT_CREATOR_ID, trident, None, None),
                LEGIT_CREATOR_ID,
            )
        };

        // registra il firmatario nell'istruzione generata
        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();
        self.accounts.creator.account_id = creator_id;

        // crea o recupera il PDA `metadata` usando seeds costanti, riproducendo la vulnerabilità
        let metadata_pubkey = fuzz_accounts.metadata.get_or_create(
            METADATA_ID,
            trident,
            Some(PdaSeeds::new(&[b"metadata"], PROGRAM_ID)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );
        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();
        self.accounts.metadata.account_id = METADATA_ID;

        // il CPI richiede sempre il programma di sistema
        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
    }
}
