use anchor_lang::{prelude::*, solana_program::bpf_loader_upgradeable};

declare_id!("EcaXg5bCYZsWjAM7wZ1xY3E7Mp95bYbur2WC5NfqpRjw");

#[program]
pub mod initialization_frontrunning {
    use super::*;

    /// Inizializza la configurazione globale ma espone una vulnerabilità di frontrunning.
    /// Qualsiasi utente può invocare per primo questa istruzione e fissare `authority` a proprio favore.
    pub fn initialize_insecure(
        ctx: Context<InitializeInsecure>,
        additional_data: u8,
    ) -> Result<()> {
        // Recupera il PDA della configurazione globale che ha un seed statico.
        let global_config = &mut ctx.accounts.global_config;

        global_config.authority = ctx.accounts.signer.key();
        global_config.additional_data = additional_data;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeInsecure<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + GlobalConfig::INIT_SPACE,
        // Seed fisso: il primo utente che chiama l'istruzione può creare il PDA senza vincoli.
        // Vulnerabilità: un attaccante può frontrunnare l'inizializzazione e diventare l'autorità legittima.
        seeds = [b"config"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug, InitSpace)]
pub struct GlobalConfig {
    // Campo sensibile: viene fissato dall'utente che riesce a inizializzare per primo il PDA.
    pub authority: Pubkey,
    pub additional_data: u8,
}
