use anchor_lang::prelude::*;

declare_id!("5wrTsnYf52JRAd8tSZDCCacQpFfLfVD64W7cHJ8FG7Ac");

#[program]
pub mod re_initialization {

    use super::*;

    pub fn secure_initialize(
        ctx: Context<Initialize>,
        parameters: InitializeParameters,
    ) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;

        if !metadata.is_initialized {
            metadata.creator = ctx.accounts.creator.key();
            metadata.name = parameters.name;
            metadata.symbol = parameters.symbol;
            metadata.uri = parameters.uri;
            metadata.year_of_creation = parameters.year_of_creation;
            metadata.is_initialized = true;
        } else {
            panic!("Account already Initialized")
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init_if_needed,
        payer=creator,
        space = 8+Metadata::LEN,
        seeds=[b"metadata"],
        bump
    )]
    pub metadata: Account<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Metadata {
    pub is_initialized: bool,
    pub creator: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub year_of_creation: u64,
}

impl Metadata {
    pub const LEN: usize = 1 + 32 + 5 + 5 + 5 + 8;
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeParameters {
    name: String,
    symbol: String,
    uri: String,
    year_of_creation: u64,
}
