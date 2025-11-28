// Registers the Account Data Matching fuzz suite with Trident and drives the generated transactions during fuzzing.

use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
// Bring in the lazily created account definitions.
mod fuzz_accounts;
// Instruction harnesses describing how to build each call.
mod instructions;
// Transaction wrappers that combine instructions with hooks.
mod transactions;
// Shared data definitions used by instructions and invariants.
mod types;
pub use transactions::*;
use crate::instructions::InitializeVaultInstruction;
use trident_fuzz::traits::{AccountsMethods, RemainingAccountsMethods};

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// for fuzzing
    trident: Trident,
    /// for storing fuzzing accounts
    fuzz_accounts: FuzzAccounts,
}

#[flow_executor]
impl FuzzTest {
        // Initialize Trident and zeroed account storage for each fuzz campaign.
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        // Seed the vault state before fuzzing so updates have something to modify.
        let mut ix = InitializeVaultTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        
        // Actually run the initialize transaction once per iteration start.
        self.trident.execute_transaction(&mut ix, Some("initialize_vault"));
        // perform any initialization here, this method will be executed
        // at start of each iteration
    }

    #[flow]
    fn flow1(&mut self) {
        // Spawn the secure update transaction harness, reusing cached accounts.
        let mut update = UpdateVaultDataSecureTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        
        // Execute the update and trigger invariant checks afterwards.
        self.trident.execute_transaction(&mut update, Some("update_vault_data_secure"));
    }

    /*#[flow]
    fn flow2(&mut self) {
        // perform logic which is meant to be fuzzed
        // Placeholder: add more flows here if you need additional coverage.
        // this flow is selected randomly from other flows
    }

    #[end]
    fn end(&mut self) {
        // perform any cleaning here, this method will be executed
        // Optional cleanup hook if the flows allocate extra resources.
        // at the end of each iteration
    }*/
}

fn main() {
    // Run 1,000 iterations with a corpus size of 100 inputs.
    FuzzTest::fuzz(1000, 100);
}
