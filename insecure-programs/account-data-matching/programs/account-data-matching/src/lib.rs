use anchor_lang::prelude::*;

declare_id!("FufZySVu1t5niHkT1Botiyssi9cgUcQtBXJQtNsiRbwM");

#[program]
pub mod account_data_matching {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, vault_data: u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        vault.vault_authority = ctx.accounts.vault_authority.key();
        vault.data = vault_data;

        Ok(())
    }

    pub fn update_vault_data_insecure(ctx: Context<UpdateVaultAuthorityInsecure>, new_data: u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        vault.data = new_data;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(
        init,
        payer = vault_authority,
        space = 8 + Vault::LEN,
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateVaultAuthorityInsecure<'info> {
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(
        mut,
    )]
    pub vault: Account<'info, Vault>,
}

#[account]
pub struct Vault {
    pub vault_authority: Pubkey,
    pub data: u8
}

impl Vault {//       33 + 1
    const LEN: usize = 33;
}

#[error_code]
pub enum AccountDataMatchingError {
    #[msg("Signer doesn't match the current vault authority!")]
    UnauthorizedVaultDataUpdate,
}
