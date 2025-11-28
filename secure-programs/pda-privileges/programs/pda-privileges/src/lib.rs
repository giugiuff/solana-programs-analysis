use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

declare_id!("J6SWoen6bHQQEnPpU8TUiU46ABpthjcvrYeiXCcUXa17");

#[program]
pub mod pda_privileges {

    use super::*;

    // Inizializza il vault e l'account di metadata
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let metadata_account = &mut ctx.accounts.metadata_account;
        // Imposta il creatore dell'account di metadata
        metadata_account.creator = ctx.accounts.vault_creator.key();
        Ok(())
    }

    // Prelievo sicuro dal vault
    pub fn secure_withdraw(ctx: Context<SecureWithdraw>) -> Result<()> {
        // Ottieni l'importo da prelevare
        let amount = ctx.accounts.vault.amount;
        let metadata_account = &mut ctx.accounts.metadata_account;

        // Definisci i signer seeds per la PDA (Program Derived Address)
        let signer_seeds: &[&[&[u8]]] = &[&[b"metadata_account", metadata_account.creator.as_ref(), &[ctx.bumps.metadata_account]]];

        // Crea il contesto CPI per il trasferimento di token
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.withdraw_destination.to_account_info(),
                authority: metadata_account.to_account_info(),
            },
            signer_seeds,
        );

        // Esegui il trasferimento di token
        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    // Il creatore del vault (firmatario) della transazione
    #[account(mut)]
    pub vault_creator: Signer<'info>,
    // L'account token che rappresenta il vault
    #[account(
        init,
        payer = vault_creator,
        associated_token::mint = mint,
        associated_token::authority = metadata_account,
    )]
    pub vault: Account<'info, TokenAccount>,
    // L'account di metadata, che memorizza le informazioni sul creatore
    #[account(
        init,
        payer = vault_creator,
        space = 8 + MetadataAccount::LEN,
        seeds = [b"metadata_account",vault_creator.key().as_ref()],
        bump,
    )]
    pub metadata_account: Account<'info, MetadataAccount>,
    // L'account della mint
    pub mint: Account<'info, Mint>,
    // Account del programma di sistema
    pub system_program: Program<'info, System>,
    // Account del programma token
    pub token_program: Program<'info, Token>,
    // Account del programma di token associati
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    // Il creatore (firmatario) della transazione
    pub creator: Signer<'info>,
    // L'account token che rappresenta il vault
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = metadata_account,
    )]
    pub vault: Account<'info, TokenAccount>,
    // L'account token di destinazione per il prelievo
    #[account(
        mut,
        token::mint = mint,
    )]
    pub withdraw_destination: Account<'info, TokenAccount>,
    // L'account di metadata, che memorizza le informazioni sul creatore e ne verifica la corrispondenza
    #[account(
        seeds = [b"metadata_account",metadata_account.creator.key().as_ref()],
        bump,
        has_one = creator,
    )]
    pub metadata_account: Account<'info, MetadataAccount>,
    // L'account della mint
    pub mint: Account<'info, Mint>,
    // Account del programma token
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct MetadataAccount {
    pub creator: Pubkey,
}

impl MetadataAccount {
    const LEN: usize = 32;
}
