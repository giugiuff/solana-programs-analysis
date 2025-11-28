use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account as SPLTokenAccount;

declare_id!("4u6h9QAMT8TuXauVZa9ieeQext18EtjecnX95xxw4xaa");

#[program]
pub mod ownership_check {
    use super::*;

    // Registrazione non sicura del saldo del token, versione 1
    //  registra il saldo di un account token accedendo direttamente al campo `amount`.
    pub fn secure_log_balance_v1(ctx: Context<SecureOwnershipv1>) -> Result<()> {
        msg!(
            "The balance: {} of Token Account: {} corresponds to owner: {} and Mint: {}",
            ctx.accounts.token_account.amount,
            ctx.accounts.token_account.key(),
            ctx.accounts.token_account_owner.key(),
            ctx.accounts.mint.key(),
        );
        Ok(())
    }
    // Registrazione non sicura del saldo del token, versione 2
    // registra il saldo di un account token scompattando i dati dell'account.
    pub fn secure_log_balance_v2(ctx: Context<SecureOwnershipv2>) -> Result<()> {
        msg!(
            "The balance: {} of Token Account: {} corresponds to owner: {} and Mint: {}",
            ctx.accounts.token_account.amount,
            ctx.accounts.token_account.key(),
            ctx.accounts.token_account_owner.key(),
            ctx.accounts.mint.key(),
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SecureOwnershipv1<'info> {
    
    pub mint: Account<'info, Mint>,
    //il token account con i contraints per assicurare il corretto mint e owner
    #[account(
        token::authority = token_account_owner,
        token::mint = mint
    )]
    pub token_account: Account<'info, TokenAccount>,
    // proprietario token account
    pub token_account_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureOwnershipv2<'info> {
    // The mint account
    pub mint: Account<'info, Mint>,
    // il token account con associated token constraints per assicurare il corretto mint e owner
    #[account(
        associated_token::authority = token_account_owner,
        associated_token::mint = mint
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    pub token_account_owner: Signer<'info>,
}
