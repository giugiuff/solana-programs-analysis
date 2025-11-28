// =========================
// use & imports
// =========================
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

const MYDATA_ENCODED_LEN: usize = 8 + 1; 

// =========================
// error.rs (ERRORI SPECIFICI)
// =========================
// Variante senza dipendenze esterne (thiserror non necessaria)
#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MyError {
    NotWritable = 0,
    NotAuthorized = 1,
    InvalidData = 2,
}

impl From<MyError> for ProgramError {
    fn from(e: MyError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// =========================
// state.rs  (STATO ON-CHAIN)
// =========================
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct MyData {
    pub counter: u64,
    pub flag: bool,
}

// =========================
// instruction.rs (ISTRUZIONI)
// =========================
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum MyInstruction {
    /// Inizializza lo stato con i valori forniti (tipicamente su account appena creato)
    Initialize { counter: u64, flag: bool },
    /// Aggiorna i campi esistenti con i nuovi valori
    Update { counter: u64, flag: bool },
}

// =========================
// processor.rs (LOGICA)
// =========================
fn process(program_id: &Pubkey, accounts: &[AccountInfo], ix_data: &[u8]) -> ProgramResult {
    // 1) Decode istruzione
    let ix = MyInstruction::try_from_slice(ix_data).map_err(|_| {
        msg!("Decodifica istruzione fallita (Borsh)");
        MyError::InvalidData
    })?;

    // 2) Parsing account list (qui: un solo account di stato)
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // 3) Ownership & Writability
    if account.owner != program_id {
        msg!("L'account target non appartiene a questo programma");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !account.is_writable {
        msg!("L'account target non è writable");
        return Err(MyError::NotWritable.into());
    }

    // 4) Routing logico
    match ix {
        MyInstruction::Initialize { counter, flag } => {
            let data = MyData { counter, flag };

            // Verifica capienza buffer prima di scrivere
            if account.data_len() < MYDATA_ENCODED_LEN {
                msg!("Buffer account insufficiente: {} < {}", account.data_len(), MYDATA_ENCODED_LEN);
                return Err(ProgramError::AccountDataTooSmall);
            }

            // Scrittura iniziale
            data.serialize(&mut &mut account.data.borrow_mut()[..]).map_err(|_| {
                msg!("Serializzazione iniziale fallita");
                MyError::InvalidData
            })?;

            msg!("Initialize: stato scritto {:?}", data);
        }

        MyInstruction::Update { counter, flag } => {
            // Deserializza stato esistente
            let mut current = MyData::try_from_slice(&account.data.borrow()).map_err(|_| {
                msg!("Deserializzazione stato esistente fallita");
                MyError::InvalidData
            })?;

            // Aggiorna in memoria
            current.counter = counter;
            current.flag = flag;

            // Verifica capienza e riserializza
             if account.data_len() < MYDATA_ENCODED_LEN {
                msg!("Buffer account insufficiente: {} < {}", account.data_len(), MYDATA_ENCODED_LEN);
                return Err(ProgramError::AccountDataTooSmall);
            }

            current.serialize(&mut &mut account.data.borrow_mut()[..]).map_err(|_| {
                msg!("Serializzazione aggiornamento fallita");
                MyError::InvalidData
            })?;

            msg!("Update: nuovo stato {:?}", current);
        }
    }

    Ok(())
}

// =========================
// entrypoint.rs (ENTRYPOINT)
// =========================
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Program start");
    let res = process(program_id, accounts, instruction_data);
    if res.is_ok() {
        msg!("Program end: OK");
    } else {
        msg!("Program end: ERROR");
    }
    res
}
