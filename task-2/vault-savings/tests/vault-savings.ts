import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultSavings } from "../target/types/vault_savings";

describe("vault-savings", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vaultSavings as Program<VaultSavings>;

  /*const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
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
  );*/

  const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), provider.publicKey.toBytes()], // User's public key
    program.programId
  )[0];

  // Derive the vault PDA using seeds ["vault", vaultState PDA]
  const vaultSavingsAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBytes()], // Vault state PDA
    program.programId
  )[0];

  console.log("Vault State PDA:", vaultState.toBase58());
  console.log("Vault PDA:", vaultSavingsAccount.toBase58());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL))
      .accountsPartial({
        user: provider.publicKey,
        state: vaultState,
        vaultSavingsAccount: vaultSavingsAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Deposits 3 SOL!", async () => {
    const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), provider.publicKey.toBytes()], // User's public key
      program.programId
    )[0];

    // Derive the vault PDA using seeds ["vault", vaultState PDA]
    const vaultSavingsAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultState.toBytes()], // Vault state PDA
      program.programId
    )[0];

    console.log("Vault State PDA:", vaultState.toBase58());
    console.log("Vault PDA:", vaultSavingsAccount.toBase58());

    // Add your test here.
    const tx = await program.methods
      .deposit(new anchor.BN(3 * anchor.web3.LAMPORTS_PER_SOL))
      .accountsPartial({
        user: provider.publicKey,
        state: vaultState,
        vaultSavingsAccount: vaultSavingsAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
