// Re-exports the instruction harnesses that exercise the Ownership Check program logic under fuzzing.

pub mod secure_log_balance_v_1;
pub mod secure_log_balance_v_2;
pub use secure_log_balance_v_1::*;
pub use secure_log_balance_v_2::*;
