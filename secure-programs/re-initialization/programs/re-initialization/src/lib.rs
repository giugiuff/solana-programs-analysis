use anchor_lang::prelude::*;

// Programma che protegge un metadata PDA da re-inizializzazioni dopo la prima popolazione.

declare_id!("5wrTsnYf52JRAd8tSZDCCacQpFfLfVD64W7cHJ8FG7Ac");

#[program]
pub mod re_initialization {

    use super::*;

    // Unico entrypoint: popola i campi metadata solo alla prima invocazione.
    pub fn secure_initialize(
        ctx: Context<Initialize>,
        parameters: InitializeParameters,
    ) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;

        if !metadata.is_initialized {
            // Prima inizializzazione: salva il signer e le stringhe fornite.
            metadata.creator = ctx.accounts.creator.key();
            metadata.name = parameters.name;
            metadata.symbol = parameters.symbol;
            metadata.uri = parameters.uri;
            metadata.year_of_creation = parameters.year_of_creation;
            metadata.is_initialized = true;
        } else {
            // Tentativi successivi vengono respinti per evitare re-inizializzazioni.
            panic!("Account already Initialized")
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Signer che paga la creazione del PDA.
    #[account(mut)]
    pub creator: Signer<'info>,
    // Metadata PDA riutilizzato fra le chiamate grazie a seeds e bump fissi.
    #[account(
        init_if_needed,
        payer=creator,
        space = 8+Metadata::LEN,
        seeds=[b"metadata"],
        bump
    )]
    pub metadata: Account<'info, Metadata>,
    // Necessario per pagare la rent e allocare il PDA.
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
