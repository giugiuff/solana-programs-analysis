// write_account.ts (CLI/Node.js) 
// Initialize: 0x00 | u64 LE | u8
// Update:     0x01 | u64 LE | u8

import {
  Transaction,
  TransactionInstruction,
  PublicKey,
  Connection,
  Keypair,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'fs';

// CONFIG
const PROGRAM_ID = new PublicKey('B4s33GM1ukdLjAy4vdUUDguPe7vVzXr8SVrKYeohQJ1L');
const DATA_ACCOUNT_PUBKEY = new PublicKey('DNGFZ8g2SJnvV9nh6L7L4V7EmNzdjYwHVW42LaYPvcDv');

// Helpers: serializzazione enum Borsh "a mano"
function encodeU64LE(n: number | bigint): Uint8Array {
  const buf = new ArrayBuffer(8);
  const view = new DataView(buf);
  view.setBigUint64(0, BigInt(n), true);
  return new Uint8Array(buf);
}
function encodeBool(b: boolean): Uint8Array {
  return Uint8Array.of(b ? 1 : 0);
}

// MyInstruction::Initialize { counter, flag }
function serializeInitialize(counter: number | bigint, flag: boolean): Uint8Array {
  const discr = Uint8Array.of(0x00); // variante 0
  const c = encodeU64LE(counter);
  const f = encodeBool(flag);
  const out = new Uint8Array(1 + 8 + 1);
  out.set(discr, 0);
  out.set(c, 1);
  out.set(f, 1 + 8);
  return out;
}

// MyInstruction::Update { counter, flag }
function serializeUpdate(counter: number | bigint, flag: boolean): Uint8Array {
  const discr = Uint8Array.of(0x01); // variante 1
  const c = encodeU64LE(counter);
  const f = encodeBool(flag);
  const out = new Uint8Array(1 + 8 + 1);
  out.set(discr, 0);
  out.set(c, 1);
  out.set(f, 1 + 8);
  return out;
}

// (opzionale) util per leggere lo stato MyData dall’account: u64 LE + u8
function deserializeMyData(data: Uint8Array) {
  if (data.length < 9) throw new Error('Data troppo corta');
  const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
  const counter = view.getBigUint64(0, true);
  const flag = view.getUint8(8) === 1;
  return { counter, flag };
}

async function main() {
  // Connessione
  const connection = new Connection('http://127.0.0.1:8899', 'confirmed');
  // const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  // Wallet locale
  const secretKey = Uint8Array.from(
    JSON.parse(fs.readFileSync(`${process.env.HOME}/.config/solana/id.json`, 'utf-8'))
  );
  const payer = Keypair.fromSecretKey(secretKey);

  console.log('Wallet locale:', payer.publicKey.toBase58());
  console.log('Account dati target:', DATA_ACCOUNT_PUBKEY.toBase58());
  console.log('Program ID:', PROGRAM_ID.toBase58());

  // Scegli quale chiamare: Initialize (prima volta) o Update (successive)
  const doInitialize = true; // <-- imposta a false per testare Update

  const counter = 555n;
  const flag = true;
  const data = doInitialize
    ? serializeInitialize(counter, flag)
    : serializeUpdate(counter, flag);

  console.log(
    `Istruzione = ${doInitialize ? 'Initialize' : 'Update'}; payload (hex):`,
    Buffer.from(data).toString('hex')
  );

  // Costruisci istruzione
  const ix = new TransactionInstruction({
    keys: [{ pubkey: DATA_ACCOUNT_PUBKEY, isSigner: false, isWritable: true }],
    programId: PROGRAM_ID,
    data,
  });

  // Invia
  const tx = new Transaction().add(ix);
  const signature = await sendAndConfirmTransaction(connection, tx, [payer]);
  console.log('Transazione inviata. Signature:', signature);

  // Verifica: leggi lo stato aggiornato
  const accountInfo = await connection.getAccountInfo(DATA_ACCOUNT_PUBKEY, 'confirmed');
  if (!accountInfo) {
    console.error('Account non trovato dopo la transazione!');
    return;
  }
  const stored = new Uint8Array(accountInfo.data);
  const parsed = deserializeMyData(stored);
  console.log('Dati letti dall’account:');
  console.log('  counter:', parsed.counter.toString());
  console.log('  flag:', parsed.flag);
}

main().catch((err) => {
  console.error('Errore durante esecuzione:', err);
});
