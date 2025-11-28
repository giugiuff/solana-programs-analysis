use anchor_lang::prelude::*;

declare_id!("GWDHzthDQUjFKbosYUSpxAX5dCoByDXPrdbuXfCxPQ5B");

//================================
// entrypoint.rs (ENTRYPOINT -> #[program])
//================================

/* in anchor non serve 'entrypoint!' : le funzioni dentro #[program]
sono le istruzioni invocabili dal runtime */

#[program]
pub mod my_program_anchor {
    use super::*;

    // Inizializza un nuovo account di stato con {counter, flag}
    pub fn initialize(ctx: Context<Initialize>, counter: u64, flag: bool) -> Result<()> {
        let data = &mut ctx.accounts.data;
        data.counter = counter;
        data.flag = flag;
        Ok(())
    }

    // Aggiorna un account di stato esistentes
    pub fn update(ctx: Context<Update>, counter: u64, flag: bool) -> Result<()> {
        let data = &mut ctx.accounts.data;
        data.counter = counter;
        data.flag = flag;
        Ok(())
    }
}

// =========================
// state.rs  (STATO ON-CHAIN -> #[account])
// =========================
// Anchor usa Borsh di default e aggiunge *8 byte di discriminator*
// all’inizio di ogni account Annotato con #[account].

#[account]
pub struct MyData {
    pub counter: u64, // 8 byte
    pub flag: bool,   // 1 byte
}
// Spazio richiesto: 8 (discriminator) + 8 (u64) + 1 (bool) = 17
pub const MYDATA_SIZE: usize = 8 + 8 + 1;

// =========================
// instruction.rs  (ISTRUZIONI -> funzioni + Context)
// =========================
// In Anchor non serve un enum MyInstruction: ogni funzione in #[program]
// è un’istruzione. Gli account richiesti e i vincoli si esprimono
// in modo dichiarativo con le struct `Context<...>`.

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Crea e inizializza l'account di stato.
    // `payer` paga la rent; `space` include il discriminator.
    #[account(init, payer = payer, space = MYDATA_SIZE)]
    pub data: Account<'info, MyData>,

    // Payer della creazione; deve essere mut (pagamento) e signer.
    #[account(mut)]
    pub payer: Signer<'info>,

    // Programma di sistema richiesto per l'allocazione dell'account.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    // Aggiorniamo un account esistente.
    // Anchor verifica anche ownership e deserializzazione.
    #[account(mut)]
    pub data: Account<'info, MyData>,
}

// =========================
// error.rs  (ERRORI SPECIFICI -> #[error_code])
// =========================
// Facoltativi: se vuoi mappare errori applicativi specifici (come nel nativo).
#[error_code]
pub enum MyError {
    #[msg("Account non writable")]
    NotWritable,
    #[msg("Dati non validi")]
    InvalidData,
}