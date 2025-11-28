// Builds the secure_user_read transaction for the Type Cosplay fuzz run, pairing it with the prepared instruction accounts and data.

use crate::fuzz_accounts::FuzzAccounts;
use crate::instructions::*;
use crate::types::User;
use std::str::FromStr;
use trident_fuzz::fuzzing::*;

/// Customize transaction behavior by adding more instructions.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/multi-instruction-transactions/
#[derive(Debug, TridentTransaction, Default)]
pub struct SecureUserReadTransaction {
    pub instruction: SecureUserReadInstruction,
}

/// Methods for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-advanced/trident-transactions/transaction-methods/
impl TransactionHooks for SecureUserReadTransaction {
    type IxAccounts = FuzzAccounts;

    fn pre_transaction(&self, client: &mut impl FuzzClient) {
        const PROGRAM_ID: &str = "5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618";
        let program_id = Pubkey::from_str(PROGRAM_ID).expect("valid program id");

        let authority_pubkey = self.instruction.accounts.authority.pubkey();
        let (user_pubkey, _) =
            Pubkey::find_program_address(&[b"user", authority_pubkey.as_ref()], &program_id);

        let user_account = client.get_account(&user_pubkey);
        let user_data = user_account.data();
        if user_data.len() < 8 {
            return;
        }

        let forged_user = User::try_from_slice(&user_data[8..])
            .expect("failed to deserialize user state");

        let cosplay_pubkey = self.instruction.accounts.user.pubkey();
        let mut cosplay_account = client.get_account(&cosplay_pubkey);

        let forged_user_bytes = borsh::to_vec(&forged_user).expect("serialize forged user");

        if cosplay_account.data().len() == forged_user_bytes.len() {
            cosplay_account.set_data_from_slice(&forged_user_bytes);
            client.set_account_custom(&cosplay_pubkey, &cosplay_account);
        }
    }
}
