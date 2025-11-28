use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod instructions;
mod transactions;
mod types;
pub use transactions::*;
use crate::instructions::InitializeVaultInstruction;
use trident_fuzz::traits::{AccountsMethods, RemainingAccountsMethods};

#[derive(FuzzTestMethods)]
struct FuzzTest {
   
    trident: Trident,

    fuzz_accounts: FuzzAccounts,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        let mut ix = InitializeVaultTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        
        self.trident.execute_transaction(&mut ix, Some("initialize_vault"));
        // perform any initialization here, this method will be executed
        // at start of each iteration
    }

    #[flow]
    fn flow1(&mut self) {
        let mut update = UpdateVaultDataInsecureTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        
        self.trident.execute_transaction(&mut update, Some("update_vault_data_insecure"));
    }

    /*#[flow]
    fn flow2(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows
    }

    #[end]
    fn end(&mut self) {
        // perform any cleaning here, this method will be executed
        // at the end of each iteration
    }*/
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
