// init_account.ts
import * as anchor from "@coral-xyz/anchor";
import { Keypair, SystemProgram } from "@solana/web3.js";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import { MyProgramAnchor } from "../target/types/my_program_anchor";

async function main() {
  // 1) Connection al validator locale
  const connection = new anchor.web3.Connection("http://127.0.0.1:8899", "confirmed");

  // 2) Wallet dal file predefinito della Solana CLI
  const kpPath = path.join(os.homedir(), ".config", "solana", "id.json");
  const secret = JSON.parse(fs.readFileSync(kpPath, "utf-8"));
  const payer = Keypair.fromSecretKey(Uint8Array.from(secret));
  const wallet = new anchor.Wallet(payer);

  // 3) Provider esplicito
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: "confirmed" });
  anchor.setProvider(provider);

  // 4) Program
  const program = anchor.workspace.MyProgramAnchor as anchor.Program<MyProgramAnchor>;

  // 5) Crea account MyData e chiama initialize(counter=0, flag=false)
  const dataAccount = Keypair.generate();
  console.log("Nuovo account MyData:", dataAccount.publicKey.toBase58());

  await program.methods
    .initialize(new anchor.BN(0), false)
    .accounts({
      data: dataAccount.publicKey,
      payer: provider.wallet.publicKey,
    })
    .signers([dataAccount])
    .rpc();

  const acc = await program.account.myData.fetch(dataAccount.publicKey);
  console.log("Dati on-chain:", acc);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
