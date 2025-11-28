use anchor_lang::prelude::*;

declare_id!("5JeEqUd5HHFtPSagJM13tjN57Ry9rmkt5pNacJ53g618");

#[program]
pub mod type_cosplay {
    use super::*;

    pub fn initialize_user(
        ctx: Context<InitializeUser>,
        metadata_account: Pubkey,
        age: u32,
    ) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.authority = ctx.accounts.authority.key();
        user.metadata_account = metadata_account;
        user.age = age;
        Ok(())
    }

    pub fn initialize_user_metadata(
        ctx: Context<InitializeUserMetadata>,
        user_account: Pubkey,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        let m = &mut ctx.accounts.user_metadata;
        m.authority = ctx.accounts.authority.key();
        m.user_account = user_account;
        m.pin1 = pin1;
        m.pin2 = pin2;
        m.pin3 = pin3;
        m.pin4 = pin4;
        Ok(())
    }

    pub fn secure_user_read(ctx: Context<SecureTypeCosplay>) -> Result<()> {
        let user = &ctx.accounts.user;
        msg!(
            "The Age of the User: {} is: {}",
            ctx.accounts.authority.key(),
            user.age
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SecureTypeCosplay<'info> {
    #[account(
        has_one = authority,
    )]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
}

#[account]
pub struct User {
    pub authority: Pubkey,
    pub metadata_account: Pubkey,
    pub age: u32,
}

impl User {
    pub const LEN: usize = 32 + 32 + 4;
}

#[account]

pub struct UserMetadata {
    pub authority: Pubkey,
    pub user_account: Pubkey,
    pub pin1: u8,
    pub pin2: u8,
    pub pin3: u8,
    pub pin4: u8,
}

impl UserMetadata {
    pub const LEN: usize = 32 + 32 + 1 + 1 + 1 + 1;
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + User::LEN,
        // PDA per-utente, deterministica
        seeds = [b"user", authority.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUserMetadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + UserMetadata::LEN,
        // PDA per-utente, deterministica
        seeds = [b"user_metadata", authority.key().as_ref()],
        bump
    )]
    pub user_metadata: Account<'info, UserMetadata>,
    pub system_program: Program<'info, System>,
}
