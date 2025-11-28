# Guida al programma Solana nativo (`examples/rust0`)

Questa guida spiega come compilare e distribuire il programma Rust puro e come interagirci con gli script TypeScript presenti in `client/`.

## 1. Prerequisiti

- **Rust** + `cargo` installati tramite `rustup` (toolchain nightly non necessaria).
- **Solana CLI** ≥ `1.18.0` con supporto SBF (`solana --version`).
- **Node.js** (18 oppure 20) con `npm` o `yarn`.
- File di wallet Solana in `~/.config/solana/id.json` (creabile con `solana-keygen new`).
- Dipendenze JavaScript del progetto (`npm install` dalla cartella `examples/rust0`).

## 2. Preparare cluster locale e wallet

```bash
solana-keygen new --outfile ~/.config/solana/id.json --force
solana config set --url http://127.0.0.1:8899
solana-test-validator --reset   # in un terminale separato
solana airdrop 10               # opzionale su localnet
```

Tutti gli script TypeScript leggono il wallet della CLI, quindi assicurati che sia finanziato.

## 3. Compilare il programma Rust

Dalla root del progetto nativo:

```bash
cd examples/rust0
cargo build-sbf    # usa cargo build-bpf se stai lavorando con Solana < 1.18
```

L’artefatto principale sarà `target/deploy/rust0.so`; verrà generata anche la keypair del programma `target/deploy/rust0-keypair.json`.

Ogni volta che modifichi `src/lib.rs`, ricompila prima di ridistribuire.

## 4. Deploy sulla localnet (o devnet)

1. Recupera l’indirizzo del programma:
   ```bash
   solana address -k target/deploy/rust0-keypair.json
   ```
2. Distribuisci:
   ```bash
   solana program deploy target/deploy/rust0.so \
       --program-id target/deploy/rust0-keypair.json
   ```
3. Verifica:
   ```bash
   solana program show <PROGRAM_ID>
   ```

> **Importante:** annota `<PROGRAM_ID>` e inseriscilo nelle costanti `PROGRAM_ID` di `client/create_account.ts` e `client/write_account.ts`.

Se vuoi utilizzare devnet, cambia l’`--url` della CLI e l’endpoint `Connection` negli script TS.

## 5. Setup TypeScript/Node

Le dipendenze (tra cui `@solana/web3.js`, `ts-node` e `typescript`) sono dichiarate in `examples/rust0/package.json`.

```bash
cd examples/rust0
npm install        # oppure yarn install
```

Tutti gli script si possono eseguire via `npx ts-node client/<nome_script>.ts`.

## 6. `client/create_account.ts`: creazione account dati

Questo script:

1. Usa il wallet locale come payer.
2. Calcola i lamport necessari per uno spazio di 9 byte (`u64 + bool`) e crea un nuovo account owned dal tuo programma (`PROGRAM_ID`).
3. Stampa la public key dell’account creato: **conservala**, servirà per scrivere i dati.

Esecuzione tipica:

```bash
npx ts-node client/create_account.ts
```

Parametri da aggiornare:
- `PROGRAM_ID`: imposta l’indirizzo ottenuto al deploy.
- (Opzionale) endpoint RPC (`Connection`) se lavori su devnet.

Lo script richiede automaticamente un airdrop se il wallet non ha abbastanza lamport.

## 7. `client/write_account.ts`: inizializzazione/aggiornamento stato

Questo script invia una singola `TransactionInstruction` al programma, serializzando “a mano” l’enum `MyInstruction` definito in `src/lib.rs`.

Passaggi:

1. Imposta le costanti in testa al file:
   - `PROGRAM_ID`: come sopra.
   - `DATA_ACCOUNT_PUBKEY`: l’account creato con lo script precedente.
2. Scegli l’operazione:
   - Prima scrittura → `const doInitialize = true;`
   - Aggiornamenti successivi → `const doInitialize = false;`
3. Configura i valori di `counter` e `flag`.
4. Esegui:
   ```bash
   npx ts-node client/write_account.ts
   ```

Lo script:
- Costruisce il payload Borsh (1 byte di discriminante + 8 + 1).
- Invia la transazione e stampa la signature.
- Legge l’account per mostrarti il contenuto (`counter`, `flag`) usando la stessa codifica Borsh della struttura `MyData`.

## 8. Suggerimenti e debug

- Se ricevi `ProgramError::IncorrectProgramId`, verifica che l’account dati appartenga davvero al programma (`solana account <pubkey>`).
- Usa `solana logs` mentre invii le transazioni per vedere i `msg!` definiti in `src/lib.rs`.
- Su devnet ricorda di chiedere un airdrop con `solana airdrop 2 <pubkey> --url https://api.devnet.solana.com`.
- Quando cambi cluster, aggiorna sia `solana config get` sia gli endpoint nelle `Connection`.
