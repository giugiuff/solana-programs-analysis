use crate::fuzz_accounts::FuzzAccounts; // storage condiviso di account per il fuzzer
use crate::types::*; // costanti e struct riutilizzate dalle istruzioni
use borsh::{BorshDeserialize, BorshSerialize}; // codec utilizzato per i parametri generati
use trident_fuzz::fuzzing::*; // API e helper di Trident

#[derive(TridentInstruction, Default)]
#[program_id("BmeJbj9adPfVGT3S8JJ7uMWkDgmC9xfJPijHWrWii9Nn")]
#[discriminator([88u8, 118u8, 55u8, 242u8, 213u8, 16u8, 93u8, 219u8])]
pub struct InsecureInitializev2Instruction {
    pub accounts: InsecureInitializev2InstructionAccounts,
    pub data: InsecureInitializev2InstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InsecureInitializev2InstructionData)]
#[storage(FuzzAccounts)]
// stessi account della v1, ma l'istruzione on-chain imposta anche il flag `is_initialized`
pub struct InsecureInitializev2InstructionAccounts {
    #[account(mut, signer)]
    // firmatario della transazione Anchor (attaccante o owner legittimo)
    pub creator: TridentAccount,

    #[account(mut)]
    // account di metadata che può essere sovrascritto
    pub metadata: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    // programma di sistema richiesto durante init_if_needed
    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
// incapsula i parametri di inizializzazione generati dal fuzzer
pub struct InsecureInitializev2InstructionData {
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
impl InstructionHooks for InsecureInitializev2Instruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        // crea valori casuali per evidenziare la sovrascrittura cattiva dei metadati
        self.data.parameters.name = trident.gen_string(1);
        self.data.parameters.symbol = trident.gen_string(1);
        self.data.parameters.uri = trident.gen_string(1);
        self.data.parameters.year_of_creation = trident.gen_range(1900_u64..=2100_u64);
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // decide se la transazione viene firmata dall'attaccante o dal creatore lecito
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

        // popola i metadati del firmatario nella struttura dell'istruzione
        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();
        self.accounts.creator.account_id = creator_id;

        // crea/recupera il PDA `metadata` usando seeds costanti (punto debole anche nella v2)
        let metadata_pubkey = fuzz_accounts.metadata.get_or_create(
            METADATA_ID,
            trident,
            Some(PdaSeeds::new(&[b"metadata"], PROGRAM_ID)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );
        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();
        self.accounts.metadata.account_id = METADATA_ID;

        // aggiunge il programma di sistema alla lista degli account
        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
    }
}
