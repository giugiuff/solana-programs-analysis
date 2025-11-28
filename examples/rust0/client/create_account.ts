// create_account.ts (CLI/Node.js)
// Crea un account di dati per il tuo programma Solana

import {
  Keypair,
  SystemProgram,
  Transaction,
  PublicKey,
  Connection,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'fs';

// Program ID del tuo smart contract deployato
const PROGRAM_ID = new PublicKey('B4s33GM1ukdLjAy4vdUUDguPe7vVzXr8SVrKYeohQJ1L');

// dimensione dati: u64 (8 byte) + bool (1 byte)
const SPACE = 8 + 1;

async function main() {
  // Connessione al validator locale (oppure devnet)
  const connection = new Connection('http://127.0.0.1:8899', 'confirmed');
  // const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  // Carica il wallet locale (~/.config/solana/id.json)
  const secretKey = Uint8Array.from(
    JSON.parse(fs.readFileSync(`${process.env.HOME}/.config/solana/id.json`, 'utf-8'))
  );
  const payer = Keypair.fromSecretKey(secretKey);

  console.log('Wallet locale (payer):', payer.publicKey.toBase58());

  // Nuovo account da creare
  const dataAccount = Keypair.generate();
  console.log('Nuovo data account (da creare):', dataAccount.publicKey.toBase58());

  // Lamports minimi per esenzione rent
  const rentExemptLamports = await connection.getMinimumBalanceForRentExemption(SPACE);
  console.log('Lamports necessari per rent-exemption:', rentExemptLamports);

  // Controllo saldo del wallet
  const balance = await connection.getBalance(payer.publicKey);
  console.log('Saldo attuale wallet (lamports):', balance);

  if (balance < rentExemptLamports + 10000) {
    console.log('Saldo insufficiente, richiedo airdrop di 1 SOL...');
    const sig = await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(sig, 'confirmed');
    console.log('Airdrop completato.');
  }

  // Istruzione di creazione account
  const createIx = SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: dataAccount.publicKey,
    lamports: rentExemptLamports,
    space: SPACE,
    programId: PROGRAM_ID,
  });

  // Creiamo e inviamo la transazione
  const tx = new Transaction().add(createIx);
  const signature = await sendAndConfirmTransaction(connection, tx, [payer, dataAccount]);

  console.log('Transazione inviata. Signature:', signature);
  console.log('✅ Account dati creato con successo!');
  console.log('👉 Pubkey account dati:', dataAccount.publicKey.toBase58());
  console.log('👉 Owned by program:', PROGRAM_ID.toBase58());
}

main().catch((err) => {
  console.error('Errore durante esecuzione:', err);
});
