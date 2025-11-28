# Solana Programs Security Dataset

Repository di supporto alla tesi “Analisi delle proprietà di sicurezza negli smart contract: sviluppo, vulnerabilità e strumenti di verifica”. Contiene un data set di programmi Solana scritti con Anchor, le versioni mitigate delle stesse vulnerabilità e i comandi e gli script usati per eseguire strumenti di analisi statica e fuzzing.

## Struttura della repository

- `insecure-programs/`: 11 progetti Anchor, ognuno focalizzato su una singola vulnerabilità reale. Ogni sottocartella è un workspace Anchor completo con codice Rust in `programs/<nome>/src/lib.rs`, casi di test e una cartella `analysis-reports/` dove salvare gli output degli strumenti. All'interno è presente anche il file `Total-report.html` con una sintesi dei risultati ottenuti dai tool statici sui programmi vulnerabili.
- `secure-programs/`: le controparti mitigate dei programmi vulnerabili; illustrano le best practice discusse nella tesi. Contiene anche, `Total-report-secure.html`(controparte di `Total-report.html` dei programmi insecure) e `README-analysis-tools.md` con le istruzioni operative.
- `examples/`: due template minimi (`anchor0/`, `rust0/`) usati durante la fase di studio per spiegare il ciclo di vita dei programmi.


## Data set e obiettivi

Il data set conta 22 programmi (11 vulnerabili + 11 mitigati) tratti dalla repo di ackee blockchain (https://github.com/Ackee-Blockchain/solana-common-attack-vectors) che coprono le principali classi di bug documentate nel Capitolo 5 della tesi: controlli di autorizzazione mancanti, gestione errata di PDA, CPI arbitrari, account duplication, frontrunning in fase di init, mancanza di reload/caching corretto, ownership validation, re-initialization, revival, signer spoofing e type confusion. Ogni coppia (insicuro/mitigato) isola un singolo pattern per:

1. Verificare che gli analyzer segnalino correttamente la versione vulnerabile.
2. Misurare i falsi positivi sulla versione corretta.
3. Fornire esempi autocontenuti da usare in formazione o audit.

## Strumenti e versioni di riferimento

Le campagne sperimentali della tesi hanno usato:

- Solana Fender v0.3.0
- Solana Static Analyzer commit `e72e796` (main)
- Radar commit `3d15e9f` (main, 31/08/2025)
- Trident v0.11.0 (14/08/2025)

Le istruzioni per l’uso sono duplicate in `insecure-programs/README-analysis-tools.txt` e `secure-programs/README-analysis-tools.md`.

## Come riprodurre le analisi

1. **Preparazione**: clona il repo, installa i tool elencati e assicurati che siano nel `PATH`. Ogni progetto Anchor contiene già `Anchor.toml` e `Cargo.toml` configurati.
2. **Analisi statica**: dalla root del progetto (es. `insecure-programs/account-data-matching/`) esegui:
   - `solana_fender --program programs/account-data-matching --markdown --output analysis-reports/solana-fender.md`
   - `RUST_LOG=info rust-solana-analyzer --path programs/account-data-matching --analyze --output analysis-reports/solana-static-analyzer.md`
   - `radar -p programs/account-data-matching -o analysis-reports/radar-report.md`
   Rinomina gli output se vuoi conservare più run.
3. **Fuzzing con Trident**:
   - Compila il BPF (`cargo build-sbf --manifest-path programs/<nome>/Cargo.toml` oppure `anchor build`).
   - Aggiorna `trident-tests/Trident.toml` con l’indirizzo on-chain e il percorso assoluto della `.so`.
   - Da `trident-tests/` esegui `trident fuzz run fuzz_0`.



