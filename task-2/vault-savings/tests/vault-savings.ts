import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultSavings } from "../target/types/vault_savings";

describe("vault-savings", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vaultSavings as Program<VaultSavings>;

  const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), provider.publicKey.toBuffer()],
    program.programId[0]
  );

  const vaultSavingsAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault"),
      vaultState[0].toBytes(),
      provider.publicKey.toBuffer(),
    ],
    program.programId[0]
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
