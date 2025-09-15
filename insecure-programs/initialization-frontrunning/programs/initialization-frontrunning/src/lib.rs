use anchor_lang::{prelude::*, solana_program::bpf_loader_upgradeable};

declare_id!("EcaXg5bCYZsWjAM7wZ1xY3E7Mp95bYbur2WC5NfqpRjw");

#[program]
pub mod initialization_frontrunning {
    use super::*;

    pub fn initialize_insecure(
        ctx: Context<InitializeInsecure>,
        additional_data: u8,
    ) -> Result<()> {
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
        seeds = [b"config"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Debug, InitSpace)]
pub struct GlobalConfig {
    pub authority: Pubkey,
    pub additional_data: u8,
}
