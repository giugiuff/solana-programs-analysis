use anchor_lang::prelude::*;
use update_account::Metadata;

declare_id!("3Rp7wE8v6S5Sd6mqikGQWo1tQ2jC8DfSPm1mf31zUGgH");

#[program]
pub mod account_reloading {
    use super::*;

        #[error_code]
    pub enum ErrorCode {
    #[msg("Stato non ricaricato dopo CPI")]
    StaleData,
    }


    pub fn update_cpi_noreload(ctx: Context<UpdateCPI>, new_input: u8) -> Result<()> {
        msg!(
            "Updated Metadata input - Before: {}",
            &ctx.accounts.metadata.input
        );

        let cpi_context = CpiContext::new(
            ctx.accounts.update_account.to_account_info(),
            update_account::cpi::accounts::Update {
                authority: ctx.accounts.authority.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
            },
        );

        update_account::cpi::update(cpi_context, new_input)?;

        msg!(
            "Updated Metadata input - After: {}",
            &ctx.accounts.metadata.input
        );

        Ok(())
    }

}

#[derive(Accounts)]
pub struct UpdateCPI<'info> {
    pub authority: Signer<'info>,
    #[account{
        mut,
        seeds = [b"metadata",authority.key().as_ref()],
        seeds::program = update_account::ID,
        bump,
    }]
    pub metadata: Account<'info, Metadata>,
    pub update_account: Program<'info, UpdateAccountProgram>,
}

pub struct UpdateAccountProgram;

impl anchor_lang::Id for UpdateAccountProgram {
    fn id() -> Pubkey {
        update_account::ID
    }
}
