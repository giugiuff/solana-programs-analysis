import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import { OwnershipCheck } from "../../target/types/ownership_check";

describe("ownership-check", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.OwnershipCheck as Program<OwnershipCheck>;

  const creator = web3.Keypair.generate();
  const hacker = web3.Keypair.generate();

  before("Fund the users!", async () => {
    await airdrop(provider.connection, creator.publicKey);
    await airdrop(provider.connection, hacker.publicKey);
  });


  // x x x x x x x x x x x x x x x x x x x x x
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE BELOW
  // | | | | | | | | | | | | | | | | | | | | |
  // v v v v v v v v v v v v v v v v v v v v v
  const TOKEN_PROGRAM_ID = new web3.PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);
const ASSOCIATED_TOKEN_PROGRAM_ID = new web3.PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);
const MINT_SIZE = 82;     // byte
const DECIMALS = 6;
const MINT_AMOUNT = 1_234_567n;

const u64le = (n: bigint) => {
  const b = Buffer.alloc(8);
  b.writeBigUInt64LE(n);
  return b;
};
const deriveAta = (owner: web3.PublicKey, mint: web3.PublicKey) =>
  web3.PublicKey.findProgramAddressSync(
    [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID
  )[0];

const ixInitializeMint = (
  mint: web3.PublicKey,
  decimals: number,
  mintAuthority: web3.PublicKey,
  freezeAuthority: web3.PublicKey | null = null
) => {
  const data = Buffer.alloc(1 + 1 + 32 + 1 + 32);
  data[0] = 0; // InitializeMint
  data[1] = decimals & 0xff;
  mintAuthority.toBuffer().copy(data, 2);
  data[34] = freezeAuthority ? 1 : 0;
  (freezeAuthority ? freezeAuthority.toBuffer() : Buffer.alloc(32)).copy(data, 35);
  return new web3.TransactionInstruction({
    programId: TOKEN_PROGRAM_ID,
    keys: [
      { pubkey: mint, isSigner: false, isWritable: true },
      { pubkey: web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    data,
  });
};

const ixCreateAta = (
  payer: web3.PublicKey,
  ata: web3.PublicKey,
  owner: web3.PublicKey,
  mint: web3.PublicKey
) =>
  new web3.TransactionInstruction({
    programId: ASSOCIATED_TOKEN_PROGRAM_ID,
    keys: [
      { pubkey: payer, isSigner: true, isWritable: true },
      { pubkey: ata, isSigner: false, isWritable: true },
      { pubkey: owner, isSigner: false, isWritable: false },
      { pubkey: mint, isSigner: false, isWritable: false },
      { pubkey: web3.SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: web3.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    data: Buffer.alloc(0),
  });

const ixMintTo = (
  mint: web3.PublicKey,
  destTokenAccount: web3.PublicKey,
  authority: web3.PublicKey,
  amount: bigint
) => {
  const data = Buffer.concat([Buffer.from([7]), u64le(amount)]); // 7 = MintTo
  return new web3.TransactionInstruction({
    programId: TOKEN_PROGRAM_ID,
    keys: [
      { pubkey: mint, isSigner: false, isWritable: true },
      { pubkey: destTokenAccount, isSigner: false, isWritable: true },
      { pubkey: authority, isSigner: true, isWritable: false },
    ],
    data,
  });
};

const shouldFail = async (p: Promise<any>, label: string) => {
  let failed = false;
  try { await p; } catch { failed = true; }
  if (!failed) throw new Error(`Atteso fallimento: ${label}`);
};

// === Stato condiviso per i test ===
let mintKp: web3.Keypair;
let mintPubkey: web3.PublicKey;
let creatorAta: web3.PublicKey;

// Setup unico per tutti i test
before("Bootstrap mint + ATA creator + mintTo", async () => {
  const connection = provider.connection;

  // Crea il mint
  mintKp = web3.Keypair.generate();
  mintPubkey = mintKp.publicKey;

  const rentMint = await connection.getMinimumBalanceForRentExemption(MINT_SIZE);
  const txCreateMint = new web3.Transaction().add(
    web3.SystemProgram.createAccount({
      fromPubkey: creator.publicKey,
      newAccountPubkey: mintKp.publicKey,
      lamports: rentMint,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM_ID,
    }),
    ixInitializeMint(mintKp.publicKey, DECIMALS, creator.publicKey, null)
  );
  await provider.sendAndConfirm(txCreateMint, [creator, mintKp], { commitment: "confirmed" });

  // Crea ATA del creator
  creatorAta = deriveAta(creator.publicKey, mintKp.publicKey);
  const txCreateAta = new web3.Transaction().add(
    ixCreateAta(creator.publicKey, creatorAta, creator.publicKey, mintKp.publicKey)
  );
  await provider.sendAndConfirm(txCreateAta, [creator], { commitment: "confirmed" });

  // Minta al creator
  const txMintTo = new web3.Transaction().add(
    ixMintTo(mintKp.publicKey, creatorAta, creator.publicKey, MINT_AMOUNT)
  );
  await provider.sendAndConfirm(txMintTo, [creator], { commitment: "confirmed" });
});

// 1) SECURE V1 — vincola authority/mint via constraints
it("secure_log_balance_v1: applica i constraint (fail mismatch, ok owner corretto)", async () => {
  // Deve fallire con owner sbagliato
  await shouldFail(
    program.methods
      .secureLogBalanceV1()
      .accounts({
        mint: mintPubkey,
        tokenAccount: creatorAta,
        tokenAccountOwner: hacker.publicKey, // mismatch
      })
      .signers([hacker])
      .rpc(),
    "secureLogBalanceV1 con owner errato"
  );

  // Deve passare con owner corretto
  await program.methods
    .secureLogBalanceV1()
    .accounts({
      mint: mintPubkey,
      tokenAccount: creatorAta,
      tokenAccountOwner: creator.publicKey,
    })
    .signers([creator])
    .rpc();
});

// 2) SECURE V2 — richiede esattamente l’ATA (authority+mint)
it("secure_log_balance_v2: applica i constraint (fail mismatch, ok owner corretto)", async () => {
  // Deve fallire con owner sbagliato
  await shouldFail(
    program.methods
      .secureLogBalanceV2()
      .accounts({
        mint: mintPubkey,
        tokenAccount: creatorAta,
        tokenAccountOwner: hacker.publicKey, // mismatch
      })
      .signers([hacker])
      .rpc(),
    "secureLogBalanceV2 con owner errato"
  );

  // Deve passare con owner corretto
  await program.methods
    .secureLogBalanceV2()
    .accounts({
      mint: mintPubkey,
      tokenAccount: creatorAta,
      tokenAccountOwner: creator.publicKey,
    })
    .signers([creator])
    .rpc();
});
  // ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^
  // | | | | | | | | | | | | | | | | | | | | |
  //           ADD YOUR CODE ABOVE
  // | | | | | | | | | | | | | | | | | | | | |
  // x x x x x x x x x x x x x x x x x x x x x


});
export async function airdrop(
  connection: any,
  address: any,
  amount = 500_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    'confirmed'
  );
}
