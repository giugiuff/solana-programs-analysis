use anchor_lang::prelude::*;

declare_id!("BmeJbj9adPfVGT3S8JJ7uMWkDgmC9xfJPijHWrWii9Nn");

#[program]
pub mod re_initialization {

    use super::*;

    /// Inizializza (o re-inizializza) i metadati senza alcun controllo sullo stato precedente.
    /// Vulnerabilità: un attaccante può invocare nuovamente l'istruzione e sovrascrivere i campi.
    pub fn insecure_initializev1(
        ctx: Context<Initialize>,
        parameters: InitializeParameters,
    ) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;

        metadata.creator = ctx.accounts.creator.key();
        metadata.name = parameters.name;
        metadata.symbol = parameters.symbol;
        metadata.uri = parameters.uri;
        metadata.year_of_creation = parameters.year_of_creation;
        Ok(())
    }
    /// Variante che marca esplicitamente il flag `is_initialized` ma resta vulnerabile al rewrite.
    pub fn insecure_initializev2(
        ctx: Context<Initialize>,
        parameters: InitializeParameters,
    ) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;

        metadata.creator = ctx.accounts.creator.key();
        metadata.name = parameters.name;
        metadata.symbol = parameters.symbol;
        metadata.uri = parameters.uri;
        metadata.year_of_creation = parameters.year_of_creation;
        metadata.is_initialized = true;
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init_if_needed,
        // Seed costante: chiunque può ricreare l'account se viene chiuso.
        // L'opzione `init_if_needed` consente di reinizializzare l'account anche se già esistente.
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
    // Flag che indica se l'account è stato inizializzato; nel v1 rimane sempre falso.
    pub is_initialized: bool,
    // Autore registrato dei metadati.
    pub creator: Pubkey,
    // Campi informativi modificabili dall'attaccante in caso di re-initialization.
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub year_of_creation: u64,
}

impl Metadata { // 1 + 32 + 5 + 5 + 5 + 8
    pub const LEN: usize = 56;
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitializeParameters {
    name: String,
    symbol: String,
    uri: String,
    year_of_creation: u64,
}
