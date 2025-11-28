# Guida per l'esecuzione del programma

Questa guida spiega come compilare e distribuire il programma Solana scritto con Anchor e come interagirci tramite gli script TypeScript presenti in `scripts/`.

## 1. Prerequisiti

- **Rust** e `cargo` installati tramite `rustup`.
- **Solana CLI** ≥ `1.18.0` (controlla con `solana --version`).
- **Anchor CLI** ≥ `0.31.0` (controlla con `anchor --version`).
- **Node.js** (consigliato 18 o 20) con `npm`/`yarn`.
- Dipendenze TypeScript del progetto (installate dopo il clone).

### Installazione rapida (opzionale)


# Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked


## 2. Preparare l'ambiente JavaScript

Dalla root del progetto `examples/anchor` installa le dipendenze:

# Usa uno qualsiasi di questi comandi:
yarn install                   # se hai Yarn classic (>=1.22)
npx yarn@1.22.19 install       # forza Yarn 1 anche con Corepack attivo
# oppure
npm install


Le devDependency includono `ts-node`; se lanci gli scripts con `npx` (es. `npx ts-node ...`) non è necessario installarlo globalmente.

## 3. Configurare wallet e cluster locale

1. Genera (o rigenera) il wallet usato da Anchor/Solana:

   solana-keygen new --outfile ~/.config/solana/id.json --force
  
2. Punta la CLI al validator locale:
   
   solana config set --url http://127.0.0.1:8899
  
3. Avvia il validator in un terminale dedicato:
   
   solana-test-validator --reset
   
4. Esegui un airdrop per finanziare il wallet (non necessario se si usa il validator locale): 
   
   solana airdrop 10
   

## 4. Build e deploy del programma

Nel terminale principale (con il validator attivo):

# Compila il programma Anchor (genera IDL e types)
anchor build

# Distribuisci il programma sulla localnet
anchor deploy

Questi comandi producono:
- `target/deploy/anchor0.so` (build del programma)
- `target/idl/my_program_anchor.json`
- `target/types/my_program_anchor.ts` (tipi TypeScript usati dagli script)

Verifica che l'ID `GWDHzthDQUjFKbosYUSpxAX5dCoByDXPrdbuXfCxPQ5B` stampato da `anchor deploy` coincida con quello in `Anchor.toml` e nel codice Rust (`declare_id!`).

## 5. Eseguire gli script TypeScript

Gli script usano il provider Anchor configurato manualmente e il wallet della Solana CLI (`~/.config/solana/id.json`).

### 5.1 Inizializzare un nuovo account


npx ts-node scripts/init_account.ts

Output atteso:

- Stampa la public key del nuovo account `MyData`.
- Legge lo stato appena creato `{ counter: 0, flag: false }`.

Annota la public key restituita: servirà per l'aggiornamento.

### 5.2 Aggiornare un account esistente

1. Apri `scripts/update_account.ts`.
2. Sostituisci il valore `PublicKey(...)` con l'indirizzo restituito dallo script precedente.
3. Avvia lo script:
   
   npx ts-node scripts/update_account.ts
   
4. Dovresti vedere lo stato aggiornato (es. `{ counter: "42", flag: true }`).

## 6. Risorse utili

- Documentazione Anchor: <https://www.anchor-lang.com/docs/>
- Solana CLI Reference: <https://docs.solana.com/cli>
- Esempio IDL generata: `target/idl/my_program_anchor.json`

Con questi passaggi dovresti riuscire a compilare, distribuire e utilizzare il programma Anchor interagendo con gli script TypeScript del progetto.
