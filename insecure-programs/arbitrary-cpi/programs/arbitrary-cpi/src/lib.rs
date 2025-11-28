use anchor_lang::prelude::*;
use arbitrary_cpi_expected::cpi::accounts::{InitializeSecret, VerifyPin};
use arbitrary_cpi_expected::SecretInformation;
use program::ArbitraryCpi;

declare_id!("27KMmAJRGvicJx2BhBY8LvPhSTgVQVigDmzP9JCqYfoa");

#[program]
pub mod arbitrary_cpi {
    use super::*;

    /// Inizializza PIN
    /// Questa funzione inizializza un PIN segreto chiamando un CPI esterno (Cross-Program Invocation)
    /// Le quattro parti del PIN (pin1, pin2, pin3, pin4) vengono passate come argomenti
    pub fn initialize_secret(
        ctx: Context<InitializeSecretCPI>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        // Ottiene le informazioni dell'account del programma esterno
        let cpi_program = ctx.accounts.secret_program.to_account_info();

        // Crea il contesto degli account CPI
        let cpi_accounts = InitializeSecret {
            author: ctx.accounts.author.to_account_info(),
            secret_information: ctx.accounts.secret_information.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        // Crea un nuovo contesto CPI
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Chiama il CPI esterno per inizializzare il segreto con il PIN fornito
        arbitrary_cpi_expected::cpi::initialize_secret(cpi_ctx, pin1, pin2, pin3, pin4)?;
        msg!("PIN SET");
        Ok(())
    }

    /// Verifica PIN (insicura)
    /// verifica un PIN segreto chiamando un CPI esterno
    pub fn insecure_verify_pin(
        ctx: Context<InsecureVerifyPinCPI>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        // Ottiene le informazioni dell'account del programma esterno
        let cpi_program = ctx.accounts.secret_program.to_account_info();

        // Crea il contesto degli account CPI
        let cpi_accounts = VerifyPin {
            author: ctx.accounts.author.to_account_info(),
            secret_information: ctx.accounts.secret_information.to_account_info(),
        };

        // Crea un nuovo contesto CPI
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Chiama il CPI esterno per verificare il PIN
        arbitrary_cpi_expected::cpi::verify_pin(cpi_ctx, pin1, pin2, pin3, pin4)?;
        msg!("PIN VERIFIED");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSecretCPI<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    
    #[account(mut)]
    pub secret_information: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    
    pub secret_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InsecureVerifyPinCPI<'info> {
    pub author: Signer<'info>,
    
    pub secret_information: Account<'info, SecretInformation>,
   
    pub secret_program: AccountInfo<'info>,
}

#[error_code]
pub enum ArbitraryCPIError {
    #[msg("Incorrect CPI program ID")]
    CPIProgramIDMismatch,
}
