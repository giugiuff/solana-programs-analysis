import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountReloading } from "../target/types/account_reloading";
import { UpdateAccount } from "../target/types/update_account";


describe("account-reloading", () => {
  // Configura il client per usare il cluster locale

  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const programAccountReload = anchor.workspace.AccountReloading as Program<AccountReloading>;
  const programUpdateAccount = anchor.workspace.UpdateAccount as Program<UpdateAccount>;

  const signer = anchor.web3.Keypair.generate()

  const input1 = 15
  const input2 = 83
  const input3 = 157


  before("Prepare", async () => {
    await airdrop(provider.connection, signer.publicKey);

  });

  it("Initialize Metadata!", async () => {

    const metadata = get_metadata_addresses(signer.publicKey, programUpdateAccount.programId);

    await programUpdateAccount.methods.initialize(input1).accounts({
      authority: signer.publicKey,
      metadata: metadata,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([signer]).rpc({ commitment: "confirmed" });

  });

  it("Update Metadata!", async () => {

    const metadata = get_metadata_addresses(signer.publicKey, programUpdateAccount.programId);

    await programUpdateAccount.methods.update(input2).accounts({
      authority: signer.publicKey,
      metadata: metadata,
    }).signers([signer]).rpc({ commitment: "confirmed" });

  });

  //==================================================================

  it("Update Metadata - Without Reload using CPI!", async () => {

    const metadata = get_metadata_addresses(signer.publicKey, programUpdateAccount.programId);

    await programAccountReload.methods.updateCpiNoreload(input3).accounts({
      authority: signer.publicKey,
      metadata: metadata,
      updateAccount: programUpdateAccount.programId
    }).signers([signer]).rpc({ commitment: "confirmed" });

  });

  //=================================================================

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

function get_metadata_addresses(
  signer: anchor.web3.PublicKey,
  program_id: anchor.web3.PublicKey,
): anchor.web3.PublicKey {


  const [metadata, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      signer.toBuffer(),
    ],
    program_id
  );
  return metadata;
}
