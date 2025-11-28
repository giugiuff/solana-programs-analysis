// Re-exports the instruction harnesses that exercise the Account Data Matching program logic under fuzzing.

// Instruction harness for the initialization flow.
pub mod initialize_vault;
// Instruction harness for the secure update flow.
pub mod update_vault_data_secure;
// Re-export so transactions can import without verbose paths.
pub use initialize_vault::*;
// Re-export the secure update helpers as well.
pub use update_vault_data_secure::*;
