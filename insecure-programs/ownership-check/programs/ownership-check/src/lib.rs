use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account as SPLTokenAccount;

declare_id!("GhD5bDw7vBR7mo9ET56VNH85ThYqXTuWdebpGTpHvaoj");

#[program]
pub mod ownership_check {
    use super::*;

    // Insecure logging of token balance, version 1
    // This function logs the balance of a token account by directly accessing the `amount` field.
    pub fn insecure_log_balance_v1(ctx: Context<InsecureOwnershipv1>) -> Result<()> {
        msg!(
            "The balance: {} of Token Account: {} corresponds to owner: {} and Mint: {}",
            ctx.accounts.token_account.amount,
            ctx.accounts.token_account.key(),
            ctx.accounts.token_account_owner.key(),
            ctx.accounts.mint.key(),
        );
        Ok(())
    }

    // Insecure logging of token balance, version 2
    // This function logs the balance of a token account by unpacking the account data.
    pub fn insecure_log_balance_v2(ctx: Context<InsecureOwnershipv2>) -> Result<()> {
        let token = SPLTokenAccount::unpack(&ctx.accounts.token_account.data.borrow())?;

        msg!(
            "The balance: {} of Token Account: {} corresponds to owner: {} and Mint: {}",
            token.amount,
            ctx.accounts.token_account.key(),
            ctx.accounts.token_account_owner.key(),
            ctx.accounts.mint.key(),
        );
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InsecureOwnershipv1<'info> {
    // The mint account
    pub mint: Account<'info, Mint>,
    // The token account
    pub token_account: Account<'info, TokenAccount>,
    // The owner of the token account
    pub token_account_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct InsecureOwnershipv2<'info> {
    // The mint account
    pub mint: Account<'info, Mint>,
    /// CHECK: this is not secure as it can be whatever Account
    pub token_account: AccountInfo<'info>,
    // The owner of the token account
    pub token_account_owner: Signer<'info>,
}

