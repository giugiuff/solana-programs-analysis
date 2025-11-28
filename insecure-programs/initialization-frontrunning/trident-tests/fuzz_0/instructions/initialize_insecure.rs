use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

const PROGRAM_ID: Pubkey = pubkey!("EcaXg5bCYZsWjAM7wZ1xY3E7Mp95bYbur2WC5NfqpRjw");

#[derive(TridentInstruction, Default)]
#[program_id("EcaXg5bCYZsWjAM7wZ1xY3E7Mp95bYbur2WC5NfqpRjw")]
#[discriminator([66u8, 109u8, 130u8, 167u8, 42u8, 38u8, 232u8, 10u8])]
pub struct InitializeInsecureInstruction {
    pub accounts: InitializeInsecureInstructionAccounts,
    pub data: InitializeInsecureInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeInsecureInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeInsecureInstructionAccounts {
    #[account(mut, signer)]
    pub signer: TridentAccount,

    #[account(mut)]
    pub global_config: TridentAccount,

    pub system_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeInsecureInstructionData {
    pub additional_data: u8,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
/// Hook di Trident che prepara l'istruzione `initialize_insecure` simulando sia scenari legittimi
/// che tentativi di frontrunning.
impl InstructionHooks for InitializeInsecureInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, _trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Popola il campo `additional_data` con il valore generato nello scenario di fuzzing.
        self.data.additional_data = fuzz_accounts.initialization.additional_data;
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Scenario corrente: decide se usare l'autorità legittima o quella dell'attaccante.
        let scenario = &fuzz_accounts.initialization;

        // Crea o recupera l'account del firmatario legittimo.
        let legit_signer =
            fuzz_accounts
                .signer
                .get_or_create(scenario.legit_signer_id, trident, None, None);

        // Crea o recupera l'account del potenziale attaccante che tenta il frontrunning.
        let attacker_signer =
            fuzz_accounts
                .signer
                .get_or_create(scenario.attacker_signer_id, trident, None, None);

        // In base allo scenario, seleziona quale chiave pubblica userà la transazione.
        let signer_pubkey = if scenario.use_correct_authority {
            legit_signer
        } else {
            attacker_signer
        };

        // Registra il firmatario scelto come signer dell'istruzione e traccia l'ID usato.
        self.accounts.signer.set_address(signer_pubkey);
        self.accounts.signer.set_is_signer();
        self.accounts.signer.account_id = if scenario.use_correct_authority {
            scenario.legit_signer_id
        } else {
            scenario.attacker_signer_id
        };

        // Genera o recupera il PDA `global_config` con seed fissi, riproducendo il punto debole.
        let global_config = fuzz_accounts.global_config.get_or_create(
            scenario.global_config_id,
            trident,
            Some(PdaSeeds::new(&[b"config"], PROGRAM_ID)),
            Some(AccountMetadata::new(0, 0, solana_sdk::system_program::ID)),
        );
        self.accounts.global_config.set_address(global_config);
        self.accounts.global_config.set_is_writable();
        self.accounts.global_config.account_id = scenario.global_config_id;

        // Aggiunge il programma di sistema necessario per l'inizializzazione dell'account.
        self.accounts
            .system_program
            .set_address(solana_sdk::system_program::ID);
    }
}
