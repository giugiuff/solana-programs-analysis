use anchor_lang::prelude::*; 

declare_id!("Ekq3FZqpHQ6coawYtyeG9QB3QWkx9zQGKSBewdQvUyyE");

#[program]
pub mod arbitrary_cpi_expected {
    use super::*;

    /// Inizializza l'account che conserva il PIN segreto del chiamante e salva i valori forniti.
    pub fn initialize_secret(
        ctx: Context<InitializeSecret>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        // Ottiene un riferimento mutabile all'account PDA dove persistono i dati segreti.
        let secret_info = &mut ctx.accounts.secret_information;

        // Registra la chiave pubblica del firmatario per ricordare chi è autorizzato a verificare il PIN.
        secret_info.author = ctx.accounts.author.key();
        // Salva i quattro byte del PIN nella struttura, preservando l'ordine.
        secret_info.pin1 = pin1;
        secret_info.pin2 = pin2;
        secret_info.pin3 = pin3;
        secret_info.pin4 = pin4;
      
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    /// Confronta il PIN passato in input con quello conservato nell'account dell'utente.
    /// Consente la verifica solo al proprietario registrato durante l'inizializzazione.
    pub fn verify_pin(
        ctx: Context<VerifyPin>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        // Accede in sola lettura ai dati segreti e alla chiave del firmatario dell'istruzione.
        let secret = &ctx.accounts.secret_information;
        let signer = &ctx.accounts.author.key();

        // Impedisce che un soggetto diverso dal proprietario verifichi o apprenda il PIN.
        if secret.author != *signer {
            return err!(ArbitraryCPIExpectedError::UnprivilegedVerification);
        }

        // Delegazione alla logica di confronto byte-per-byte definita sulla struttura dei dati.
        secret.verify_pin(pin1, pin2, pin3, pin4)?;

        Ok(())
    }
}

/// Struttura di contesto usata quando si inizializza il PIN segreto.
#[derive(Accounts)]
pub struct InitializeSecret<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    // Crea e inizializza l'account PDA dove verranno memorizzate le informazioni sensibili.
    #[account(
        init,
        payer = author,
        space = 8 + SecretInformation::LEN,
        seeds = [b"secret_info",author.key().as_ref()],
        bump,
    )]
    pub secret_information: Account<'info, SecretInformation>,
    // Programma di sistema necessario per allocare e finanziare il nuovo account.
    pub system_program: Program<'info, System>,
}

/// Struttura di contesto per l'istruzione che verifica il PIN esistente.
#[derive(Accounts)]
pub struct VerifyPin<'info> {
    pub author: Signer<'info>,
    // Recupera lo stesso PDA creato durante l'inizializzazione sfruttando seeds e bump deterministici.
    #[account(
        seeds = [b"secret_info",author.key().as_ref()],
        bump,
    )]
    pub secret_information: Account<'info, SecretInformation>,
}

/// Dati persistenti che custodiscono il PIN e l'autorizzazione dell'utente.
#[account]
pub struct SecretInformation {
    // Chiave pubblica del proprietario del PIN, usata per autorizzare le verifiche.
    pub author: Pubkey,
    // Byte che rappresentano le singole cifre del PIN segreto.
    pub pin1: u8,
    pub pin2: u8,
    pub pin3: u8,
    pub pin4: u8,
}

/// Implementazione delle funzioni ausiliarie per operare sui dati segreti.
impl SecretInformation {
    // Dimensione: 32 byte per la chiave pubblica + 4 byte per le componenti del PIN.
    pub const LEN: usize = 36;

    fn verify_pin(&self, pin1: u8, pin2: u8, pin3: u8, pin4: u8) -> Result<()> {
        // Confronta in sequenza ogni byte del PIN salvato rispetto ai valori forniti.
        if self.pin1 != pin1 {
            return err!(ArbitraryCPIExpectedError::IncorrectPIN);
        }
        if self.pin2 != pin2 {
            return err!(ArbitraryCPIExpectedError::IncorrectPIN);
        }
        if self.pin3 != pin3 {
            return err!(ArbitraryCPIExpectedError::IncorrectPIN);
        }
        if self.pin4 != pin4 {
            return err!(ArbitraryCPIExpectedError::IncorrectPIN);
        }
        // Se nessun controllo fallisce, la verifica è considerata riuscita.
        Ok(())
    }
}

/// Enumerazione degli errori personalizzati restituiti dal programma.
#[error_code]
pub enum ArbitraryCPIExpectedError {
    #[msg("Incorrect PIN")]
    IncorrectPIN,
    // Errore restituito se il richiedente non è l'autore registrato del PIN.
    #[msg("Unprivileged Verification")]
    UnprivilegedVerification,
}
