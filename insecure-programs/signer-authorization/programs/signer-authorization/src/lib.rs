use anchor_lang::prelude::*;

declare_id!("9e2aBh4MXpyHAxr2sq8guL4dVXiUA3CaJquQ4pnexQha");

#[program]
pub mod signer_authorization {
  use super::*;

  /// Inizializza l'escrow impostando l'autorità che potrà modificarne i dati.
  pub fn initialize(ctx: Context<Initialize>, data: u8) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.authority = *ctx.accounts.authority.key;
    escrow.data = data;

    Ok(())
  }
  /// Aggiorna l'escrow senza verificare che il firmatario sia davvero l'autorità registrata.
  /// Vulnerabilità: qualsiasi signer che passa l'account PDA corretto può sovrascrivere `data`.
  pub fn insecure_authorization(ctx: Context<InsecureAuthorization>, data: u8) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    escrow.data = data;

    msg!("Data: {}", escrow.data);

    Ok(())
  }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(mut)]
  pub authority: Signer<'info>,
  #[account(init, payer = authority, space = 8 + Escrow::LEN, seeds = [b"escrow".as_ref()], bump)]
  pub escrow: Account<'info, Escrow>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InsecureAuthorization<'info> {
  pub authority: Signer<'info>,
  /// CHECK: This is not correct
  #[account(
    mut,
    // Non verifica che `authority` firmatario corrisponda al campo `escrow.authority`.
    // Permette un attacco in cui chiunque fornisce il PDA corretto.
    seeds = [b"escrow".as_ref()],
    bump
  )]
  pub escrow: Account<'info, Escrow>,
}

#[account]
pub struct Escrow {
  // Autorità registrata durante `initialize`; dovrebbe essere verificata nelle istruzioni future.
  pub authority: Pubkey,
  // Dato arbitrario aggiornabile, esposto alla sovrascrittura in `insecure_authorization`.
  pub data: u8,
}

impl Escrow { //33
  pub const LEN: usize = 33; // Pubkey + u8
}
