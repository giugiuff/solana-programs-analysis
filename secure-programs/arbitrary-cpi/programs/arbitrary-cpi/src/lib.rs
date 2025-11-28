use anchor_lang::prelude::*;
use arbitrary_cpi_expected::cpi::accounts::{InitializeSecret, VerifyPin};
use arbitrary_cpi_expected::SecretInformation;
use program::ArbitraryCpi;

declare_id!("67tPgGfjMJLMqH5u6h2Nf3pYfJn2qFPgxT8SYmKh3hzU");

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
    /// Questa funzione verifica un PIN segreto chiamando un CPI esterno
    /// Questa implementazione e contrassegnata come "insecure" a scopo dimostrativo
    pub fn secure_verify_pin(
        ctx: Context<SecureVerifyPinCPI>,
        pin1: u8,
        pin2: u8,
        pin3: u8,
        pin4: u8,
    ) -> Result<()> {
        // Ottiene le informazioni dell'account del programma esterno
        let cpi_program = ctx.accounts.secret_program.to_account_info();
        //security check
        if cpi_program.key() != arbitrary_cpi_expected::ID {
            return err!(ArbitraryCPIError::CPIProgramIDMismatch);
        }

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
pub struct SecureVerifyPinCPI<'info> {
    pub author: Signer<'info>,
    pub secret_information: Account<'info, SecretInformation>,
    pub secret_program: Program<'info, arbitrary_cpi_expected::program::ArbitraryCpiExpected>,
}

#[error_code]
pub enum ArbitraryCPIError {
    #[msg("Incorrect CPI program ID")]
    CPIProgramIDMismatch,
}
