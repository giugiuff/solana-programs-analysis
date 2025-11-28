use crate::fuzz_accounts::FuzzAccounts; // archivio condiviso degli account usati dal fuzzer
use borsh::{BorshDeserialize, BorshSerialize}; // codec per eventuali payload (qui vuoto)
use std::str::FromStr; // conversione verso Pubkey
use trident_fuzz::fuzzing::*; // API Trident (TridentAccount, PdaSeeds, ecc.)

#[derive(TridentInstruction, Default)]
#[program_id("BxYhDihgJZZxUXqwoqvzbfD8G1fwNFKQyF8L5SEiNCQP")]
#[discriminator([10u8, 220u8, 196u8, 138u8, 19u8, 60u8, 204u8, 130u8])]
pub struct CloseMetadataInstruction {
    pub accounts: CloseMetadataInstructionAccounts,
    pub data: CloseMetadataInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(CloseMetadataInstructionData)]
#[storage(FuzzAccounts)]
// account necessari per replicare la chiusura del metadata
pub struct CloseMetadataInstructionAccounts {
    #[account(mut, signer)]
    // firmatario che riceve i lamport dell'account chiuso
    pub creator: TridentAccount,

    #[account(mut)]
    // PDA dei metadati che viene chiuso
    pub metadata: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
// nessun campo dati: l'istruzione on-chain non riceve parametri
pub struct CloseMetadataInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CloseMetadataInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        const PROGRAM_ID: &str = "BxYhDihgJZZxUXqwoqvzbfD8G1fwNFKQyF8L5SEiNCQP";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid revival attack program id");

        // Seleziona il creatore deterministico dal pool di account e lo marca come signer/writable.
        let creator_pubkey = fuzz_accounts.creator.get_or_create(
            self.accounts.creator.account_id,
            trident,
            None,
            None,
        );
        self.accounts.creator.set_address(creator_pubkey);
        self.accounts.creator.set_is_signer();
        self.accounts.creator.set_is_writable();

        // Deriva il PDA `metadata` usando gli stessi seed dell'istruzione Anchor.
        let seeds: [&[u8]; 2] = [b"secret_metadata", creator_pubkey.as_ref()];
        let (metadata_pubkey, _) = Pubkey::find_program_address(&seeds, &program_id);

        self.accounts.metadata.set_address(metadata_pubkey);
        self.accounts.metadata.set_is_writable();

        // Registra/crea il PDA nello storage così da poterlo riutilizzare in altre istruzioni.
        fuzz_accounts.metadata.get_or_create(
            self.accounts.metadata.account_id,
            trident,
            Some(PdaSeeds::new(&seeds, program_id)),
            Some(AccountMetadata::new(0, 0, program_id)),
        );
    }
}
