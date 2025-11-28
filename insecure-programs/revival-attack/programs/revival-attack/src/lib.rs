use anchor_lang::prelude::*;

declare_id!("BxYhDihgJZZxUXqwoqvzbfD8G1fwNFKQyF8L5SEiNCQP");

#[program]
pub mod revival_attack {
    use super::*;

    /// Inizializza l'account dei metadati salvando un PIN a quattro byte.
    /// Non viene impostato alcun flag di inizializzazione, consentendo scritture future.
    pub fn initialize_metadata(
        ctx: Context<InitializeMetadata>,
        secret1: u8,
        secret2: u8,
        secret3: u8,
        secret4: u8,
    ) -> Result<()> {
        let secret_metadata = &mut ctx.accounts.metadata;

        secret_metadata.secret1 = secret1;
        secret_metadata.secret2 = secret2;
        secret_metadata.secret3 = secret3;
        secret_metadata.secret4 = secret4;

        msg!("Metadata Created");
        Ok(())
    }
    /// Chiude l'account dei metadati restituendo i lamport al creatore.
    /// Vulnerabilità: l'account viene svuotato ma resta riutilizzabile per un revival attack.
    pub fn close_metadata(ctx: Context<CloseMetadata>) -> Result<()> {
        let metadata = &mut ctx.accounts.metadata;
        let creator = &mut ctx.accounts.creator;

        metadata.remove_metadata();

        let metadata_balance = metadata.get_lamports();

        metadata.sub_lamports(metadata_balance)?;
        creator.add_lamports(metadata_balance)?;

        msg!("Metadata Removed");

        Ok(())
    }

    /// Verifica il PIN confrontando i quattro byte salvati nello stato.
    /// Se l'account è stato revivificato da un attaccante, i controlli avverranno sui nuovi dati.
    pub fn verify_pin(
        ctx: Context<VerifyPin>,
        secret1: u8,
        secret2: u8,
        secret3: u8,
        secret4: u8,
    ) -> Result<()> {
        let metadata = &ctx.accounts.metadata;
        if metadata.secret1 != secret1 {
            panic!("PIN1 Mismatch");
        }
        if metadata.secret2 != secret2 {
            panic!("PIN2 Mismatch");
        }
        if metadata.secret3 != secret3 {
            panic!("PIN3 Mismatch");
        }
        if metadata.secret4 != secret4 {
            panic!("PIN4 Mismatch");
        }

        msg!("PIN VERIFIED");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMetadata<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        // Seed costante: rende il PDA prevedibile e facile da riaprire dopo la chiusura.
        payer = creator,
        space = 8 + SecretMetadata::LEN,
        seeds=[b"secret_metadata",creator.key().as_ref()],
        bump,
    )]
    pub metadata: Account<'info, SecretMetadata>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseMetadata<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        // Riusa il PDA anche se precedentemente chiuso; non verifica che i segreti siano validi.
        seeds=[b"secret_metadata",creator.key().as_ref()],
        bump,
    )]
    pub metadata: Account<'info, SecretMetadata>,
}

#[derive(Accounts)]
pub struct VerifyPin<'info> {
    pub creator: Signer<'info>,
    #[account(
        mut,
        // L'account potrebbe essere stato ricreato da un attaccante prima della verifica.
        seeds=[b"secret_metadata",creator.key().as_ref()],
        bump,
    )]
    pub metadata: Account<'info, SecretMetadata>,
}

#[account]
pub struct SecretMetadata {
    // Proprietario registrato del PIN; non viene azzerato in `remove_metadata`.
    pub creator: Pubkey,
    // Quattro byte che rappresentano il PIN segreto.
    pub secret1: u8,
    pub secret2: u8,
    pub secret3: u8,
    pub secret4: u8,
}

impl SecretMetadata {// 32 + 4
    pub const LEN: usize = 36;
    pub fn remove_metadata(&mut self) {
        // Azzera i segreti ma non rimuove l'autorità, favorendo il revival attack.
        self.secret1 = 0;
        self.secret2 = 0;
        self.secret3 = 0;
        self.secret4 = 0;
    }
}
