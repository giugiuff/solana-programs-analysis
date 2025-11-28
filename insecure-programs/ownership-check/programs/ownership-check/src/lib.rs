use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Account as SPLTokenAccount;

declare_id!("GhD5bDw7vBR7mo9ET56VNH85ThYqXTuWdebpGTpHvaoj");

#[program]
pub mod ownership_check {
    use super::*;

    // Registrazione non sicura del saldo del token, versione 1
    // Questa funzione registra il saldo di un account token accedendo direttamente al campo `amount`.
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

    // Registrazione non sicura del saldo del token, versione 2
    // Questa funzione registra il saldo di un account token scompattando i dati dell'account.
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
    // L'account della mint
    pub mint: Account<'info, Mint>,
    // L'account del token
    pub token_account: Account<'info, TokenAccount>,
    // Il proprietario dell'account token
    pub token_account_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct InsecureOwnershipv2<'info> {
    // The mint account
    pub mint: Account<'info, Mint>,
    /// CHECK: non è sicuro perché può essere un Account qualsiasi
    pub token_account: AccountInfo<'info>,
    // Il proprietario dell'account token
    pub token_account_owner: Signer<'info>,
}
