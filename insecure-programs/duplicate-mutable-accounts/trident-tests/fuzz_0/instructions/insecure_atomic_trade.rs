use crate::fuzz_accounts::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

const PROGRAM_ID: Pubkey = pubkey!("2bSdoVHibWNGdDRZrd3bYJpSNyEMu2DRVpgPPajdoU24");
const VAULT_ACCOUNT_SPACE: usize = 8 + 32 + 8;

#[derive(TridentInstruction, Default)]
#[program_id("2bSdoVHibWNGdDRZrd3bYJpSNyEMu2DRVpgPPajdoU24")]
#[discriminator([18u8, 39u8, 8u8, 26u8, 153u8, 19u8, 158u8, 194u8])]
pub struct InsecureAtomicTradeInstruction {
    pub accounts: InsecureAtomicTradeInstructionAccounts,
    pub data: InsecureAtomicTradeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InsecureAtomicTradeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InsecureAtomicTradeInstructionAccounts {
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
pub struct InsecureAtomicTradeInstructionData {
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
/// Hook di Trident che prepara dati e account dell'istruzione di trade atomico durante il fuzzing.
impl InstructionHooks for InsecureAtomicTradeInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(&mut self, _trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Recupera l'importo di scambio dal contesto di fuzzing e lo imposta nell'istruzione.
        self.data.transfer_amount = fuzz_accounts.atomic_trade.transfer_amount;
    }

    fn set_accounts(&mut self, trident: &mut Trident, fuzz_accounts: &mut Self::IxAccounts) {
        // Scenario corrente definito dal fuzzer: include flag per vault duplicati e identificativi vari.
        let scenario = &fuzz_accounts.atomic_trade;

        // Prepara il firmatario A: crea/recupera l'account e lo registra come signer della transazione.
        let signer_a_pubkey = fuzz_accounts
            .signer_a
            .get_or_create(scenario.signer_a_id, trident, None, None);
        self.accounts.signer_a.set_address(signer_a_pubkey);
        self.accounts.signer_a.set_is_signer();
        self.accounts.signer_a.account_id = scenario.signer_a_id;

        // Se si vuole simulare il bug, riusa lo stesso firmatario; altrimenti imposta il firmatario B separato.
        let signer_b_pubkey = if scenario.duplicate_vaults {
            signer_a_pubkey
        } else {
            fuzz_accounts
                .signer_b
                .get_or_create(scenario.signer_b_id, trident, None, None)
        };
        self.accounts.signer_b.set_address(signer_b_pubkey);
        self.accounts.signer_b.set_is_signer();
        // Sincronizza l'identificativo dell'account B con quello A quando si forza la duplicazione.
        self.accounts.signer_b.account_id = if scenario.duplicate_vaults {
            scenario.signer_a_id
        } else {
            scenario.signer_b_id
        };

        // Seeds deterministici per calcolare la PDA del vault collegata al firmatario A.
        let signer_a_bytes = signer_a_pubkey.to_bytes();
        let vault_a_seeds: [&[u8]; 2] = [b"vault", signer_a_bytes.as_ref()];

        // Crea o recupera il vault A garantendo spazio e lamport necessari per il test.
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

        // In modalità duplicata il vault B punta volutamente allo stesso indirizzo di vault A.
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
        // Allinea l'ID di vault B a quello di vault A per tracciare che condividono lo stesso account.
        self.accounts.vault_b.account_id = if scenario.duplicate_vaults {
            scenario.vault_a_id
        } else {
            scenario.vault_b_id
        };

        // Seeds per il vault delle commissioni, condiviso da tutte le esecuzioni.
        let fee_vault_seeds: [&[u8]; 1] = [b"fee_vault"];
        // Crea/recupera il vault delle commissioni e lo marca come mutabile.
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


