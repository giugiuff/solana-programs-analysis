use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use crate::types::*;
use borsh::BorshDeserialize;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct InsecureUserReadTransaction {
    pub instruction: InsecureUserReadInstruction,
}

impl TransactionHooks for InsecureUserReadTransaction {
    type IxAccounts = FuzzAccounts;

    fn pre_transaction(&self, client: &mut impl FuzzClient) {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        let authority_pubkey = self.instruction.accounts.authority.pubkey();
        let (user_pubkey, _) =
            Pubkey::find_program_address(&[b"user", authority_pubkey.as_ref()], &program_id);
        let (metadata_pubkey, _) = Pubkey::find_program_address(
            &[b"user_metadata", authority_pubkey.as_ref()],
            &program_id,
        );

        let user_account = client.get_account(&user_pubkey);
        let user_data = user_account.data();
        if user_data.len() < 8 {
            return;
        }

        let mut forged_user =
            User::try_from_slice(&user_data[8..]).expect("failed to deserialize user state");
        forged_user.metadata_account.set_pubkey(metadata_pubkey);

        let cosplay_pubkey = self.instruction.accounts.user.pubkey();
        let mut cosplay_account = client.get_account(&cosplay_pubkey);

        let forged_user_bytes = borsh::to_vec(&forged_user).expect("serialize forged user");

        if cosplay_account.data().len() == forged_user_bytes.len() {
            cosplay_account.set_data_from_slice(&forged_user_bytes);
            client.set_account_custom(&cosplay_pubkey, &cosplay_account);
        }
    }

    fn transaction_invariant_check(&self) -> Result<(), FuzzingError> {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        let authority_pubkey = self.instruction.accounts.authority.pubkey();

        let (expected_user_pubkey, _) =
            Pubkey::find_program_address(&[b"user", authority_pubkey.as_ref()], &program_id);
        let (metadata_pubkey, _) = Pubkey::find_program_address(
            &[b"user_metadata", authority_pubkey.as_ref()],
            &program_id,
        );

        let supplied_user_pubkey = self.instruction.accounts.user.pubkey();

        if supplied_user_pubkey != expected_user_pubkey {
            return Err(FuzzingError::with_message(&format!(
                "Type cosplay detected: expected user {}, got {}",
                expected_user_pubkey, metadata_pubkey
            )));
        }

        Ok(())
    }
}
