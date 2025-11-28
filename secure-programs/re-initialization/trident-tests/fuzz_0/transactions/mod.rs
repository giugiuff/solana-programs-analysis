// Groups the transaction builders used by Trident when fuzzing the Re-Initialization scenario.

pub mod secure_initialize;

use borsh::BorshDeserialize;
use trident_fuzz::fuzzing::*;

use crate::types::{Metadata, LEGIT_CREATOR_ID};

pub use secure_initialize::*;

//invariant
pub fn assert_metadata_creator_immutable(
    metadata_account: &TridentAccount,
    actor_id: u8,
) -> Result<(), FuzzingError> {
    if actor_id == LEGIT_CREATOR_ID {
        return Ok(());
    }

    let after_snapshot = metadata_account.get_snapshot_after();
    let after_data = after_snapshot.data();

    if after_data.len() <= 8 {
        return Ok(());
    }

    let after_state = Metadata::try_from_slice(&after_data[8..]).map_err(|_| {
        FuzzingError::with_message("failed to deserialize metadata after execution")
    })?;

    let before_snapshot = metadata_account.get_snapshot_before();
    let before_data = before_snapshot.data();

    if before_data.len() <= 8 {
        return Ok(());
    }

    let before_state = Metadata::try_from_slice(&before_data[8..]).map_err(|_| {
        FuzzingError::with_message("failed to deserialize metadata before execution")
    })?;

    let before_creator = before_state.creator.get_pubkey();
    let after_creator = after_state.creator.get_pubkey();

    if before_creator != Pubkey::default() && before_creator != after_creator {
        return Err(FuzzingError::with_message(
            "metadata creator changed after initialization",
        ));
    }

    Ok(())
}
