import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";
import { assert } from 'chai';

import { Program } from "@coral-xyz/anchor";
import { ArbitraryCpi } from "../../target/types/arbitrary_cpi";
import { ArbitraryCpiExpected } from "../../target/types/arbitrary_cpi_expected";
import { ArbitraryCpiHacked } from "../../target/types/arbitrary_cpi_hacked";


describe("arbitrary-cpi", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program_cpi = anchor.workspace.ArbitraryCpi as Program<ArbitraryCpi>;
  const program_expected = anchor.workspace.ArbitraryCpiExpected as Program<ArbitraryCpiExpected>;
  const program_hacked = anchor.workspace.ArbitraryCpiHacked as Program<ArbitraryCpiHacked>;


  const creator = web3.Keypair.generate();
  const hacker = web3.Keypair.generate();


  before("Fund the users!", async () => {
    await airdrop(provider.connection, creator.publicKey);
    await airdrop(provider.connection, hacker.publicKey);

  });

  //===================================================================

  it("Initialize Secret", async () => {
    const [secretInformation, mBump] = deriveSecretAddress(creator.publicKey, program_expected.programId);

    await program_cpi.methods.initializeSecret(0, 1, 2, 5).accounts({
      author: creator.publicKey,
      secretInformation: secretInformation,
      systemProgram: web3.SystemProgram.programId,
      secretProgram: program_expected.programId
    }).signers([creator]).rpc({ commitment: "confirmed" });

  });

  it("Secure Verify Secret by creator (secure handler)", async () => {
  const [secretInformation] = deriveSecretAddress(
    creator.publicKey,
    program_expected.programId
  );

  await program_cpi.methods
    .secureVerifyPin(0, 1, 2, 5) 
    .accounts({
      author: creator.publicKey,
      secretInformation: secretInformation,
      secretProgram: program_expected.programId, 
    })
    .signers([creator])
    .rpc({ commitment: "confirmed" });
});

it("Secure: hacker can't use his program by CPI", async () => {
  const [secretInformation] = deriveSecretAddress(
    creator.publicKey,
    program_expected.programId
  );

  try {
    await program_cpi.methods
      .secureVerifyPin(0, 0, 0, 0)
      .accounts({
        author: hacker.publicKey,
        secretInformation,
        secretProgram: program_hacked.programId, 
      })
      .signers([hacker])
      .rpc({ commitment: "confirmed" });

    assert.fail("La tx doveva fallire");
  } catch (e: any) {
    const logs = e?.logs || e?.error?.logs || [];
    let code = "";
    try {
      const parsed = anchor.AnchorError.parse(logs);
      code = parsed?.error?.errorCode?.code || "";
    } catch {}

    // Accetta le varianti che compaiono a seconda di versione/config
    const msg = (e?.message || "").toLowerCase();
    assert.isTrue(
      code === "ConstraintProgram" ||
      code === "CPIProgramIDMismatch" ||
      code === "InvalidProgramId" ||
      msg.includes("invalid program id") ||
      msg.includes("constraint program"),
      `Errore inatteso: ${code || e?.message}`
    );
  }
});


  //=======================================================0


});

async function airdrop(
  connection: any,
  address: any,
  amount = 500_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    'confirmed'
  );
}

function deriveSecretAddress(creator: web3.PublicKey, programId: web3.PublicKey): [web3.PublicKey, number] {
  return web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("secret_info"),
      creator.toBuffer()
    ], programId);
}

async function getTx(signature: string, connection: web3.Connection): Promise<anchor.web3.VersionedTransactionResponse> {
  return connection.getTransaction(signature, { commitment: "confirmed", maxSupportedTransactionVersion: 2 });
}
