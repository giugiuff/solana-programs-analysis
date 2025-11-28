Strumenti di Analisi Statica e Fuzzing
====================================

Versioni utilizzate nei test della repo:
- Solana Fender: v0.3.0
- Solana Static Analyzer: commit e72e796 (branch main, repository 0xRustPro/solana-static-analyzer)
- Radar: commit 3d15e9f (branch main, repository Auditware/radar, 31 agosto 2025)
- Trident: v0.11.0 (14 agosto 2025)

Le istruzioni seguenti presumono che ogni tool sia installato e disponibile nel PATH. Esegui i comandi dalla directory del programma che vuoi analizzare (ad esempio ~/solana-programs-analysis/insecure-programs/NOME_PROGETTO o la variante secure corrispondente).

Solana Fender
-------------
Genera un report Markdown per una directory di programma:
  solana_fender --program programs/NOME_PROGRAMMA --markdown --output analysis-reports/solana-fender.md

Flag utili:
  --ignore-low / --ignore-medium / --ignore-high / --ignore-critical  Ignora i finding per severità
  --debug                                                            Stampa output dettagliato

Rust Solana Analyzer
--------------------
Esegui l'analizzatore con logging opzionale e salva il report:
  RUST_LOG=info rust-solana-analyzer --path programs/NOME_PROGRAMMA --analyze --output analysis-reports/solana-static-analyzer.md

Suggerimenti:
  Ometti --output se preferisci ricevere il report su stdout.
  Imposta RUST_LOG=debug per più diagnostica.

Radar
-----
Genera report in Markdown (o JSON/SARIF cambiando estensione) e opzionalmente arresta i container quando termina:
  radar -p programs/NOME_PROGRAMMA -o analysis-reports/radar-report.md
  radar -p programs/NOME_PROGRAMMA -o analysis-reports/radar-report.json
  radar -p programs/NOME_PROGRAMMA -o analysis-reports/radar-report.md -d    # arresta i container al termine

Opzioni utili:
  -s SUBPERCORSO     Analizza solo una sottodirectory o un file
  -t CARTELLA_RULES  Aggiunge regole custom
  -i low,medium      Ignora finding per severità
  -a                 Esporta anche l'AST

Nota: ogni esecuzione sovrascrive il file di destinazione, quindi rinominalo se vuoi mantenere run multiple.

Trident
-------
Trident fornisce fuzzing guidato dalla copertura per programmi Solana.

Passaggi tipici:
  1. Installa la CLI (una sola volta):
       cargo install --locked trident-cli
     Usa --force per aggiornare alla versione più recente.

  2. Compila il programma BPF che vuoi fuzzare per avere un .so aggiornato:
       cargo build-sbf --manifest-path programs/NOME_PROGRAMMA/Cargo.toml
     Se usi Anchor, sostituisci con anchor build.

  3. Configura Trident.toml dentro trident-tests con l'indirizzo on-chain del programma e il percorso assoluto della .so appena costruita.

  4. Avvia il fuzzer da trident-tests:
       trident fuzz run fuzz_0
     Sostituisci, se necessario, fuzz_0 con il target definito nel Cargo.toml e aggiungi --with-exit-code per far fallire il comando al rilevamento di violazioni. Output e metriche finiscono in trident-output/ per default.

  5. Per riprodurre un seed specifico:
       trident fuzz debug fuzz_0 <SEED>

Per un giro veloce senza CLI puoi eseguire:
  cargo run --release --bin fuzz_0
Il numero di iterazioni è definito nel template (FuzzTest::fuzz).

