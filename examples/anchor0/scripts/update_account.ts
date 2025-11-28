// scripts/update_account.ts
import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import { MyProgramAnchor } from "../target/types/my_program_anchor";

async function main() {
  // 1) Connessione al validator locale
  const connection = new anchor.web3.Connection("http://127.0.0.1:8899", "confirmed");

  // 2) Wallet dalla key della Solana CLI (~/.config/solana/id.json)
  const kpPath = path.join(os.homedir(), ".config", "solana", "id.json");
  const secret = JSON.parse(fs.readFileSync(kpPath, "utf-8"));
  const payer = Keypair.fromSecretKey(Uint8Array.from(secret));
  const wallet = new anchor.Wallet(payer);

  // 3) Provider esplicito
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  anchor.setProvider(provider);

  // 4) Program dal workspace (lancia da root del progetto)
  const program = anchor.workspace.MyProgramAnchor as anchor.Program<MyProgramAnchor>;

  //Viene inserito il PubKey dell'account creto dall'init
  const dataPubkey = new PublicKey("E3s95AX14KR82o26YsxAyTvXD1dJmF35XthAh3gdETqa");

  // Esempio update: counter=42, flag=true
  await program.methods
    .update(new anchor.BN(42), true)
    .accounts({ data: dataPubkey }) 
    .rpc();

  const acc = await program.account.myData.fetch(dataPubkey);
  console.log("Dati aggiornati:", { counter: acc.counter.toString(), flag: acc.flag });
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
