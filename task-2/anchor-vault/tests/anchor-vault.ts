import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env(); // Load the default provider from the Anchor environment (usually your local wallet and devnet/localnet RP
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  const userPublicKey = anchor.getProvider().wallet.publicKey;

  // Derive the vault state PDA using seeds ["state", user public key]
  const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), provider.publicKey.toBytes()], // User's public key
    program.programId
  )[0];

  // Derive the vault PDA using seeds ["vault", vaultState PDA]
  const vault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBytes()], // Vault state PDA
    program.programId
  )[0];

  console.log("Vault State PDA:", vaultState.toBase58());
  console.log("Vault PDA:", vault.toBase58());

  it("Is initialized with rent exemption!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accountsPartial({
        vaultState: vaultState,
        vault: vault,
        user: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    console.log(
      "Your vault info",
      await provider.connection.getAccountInfo(vault)
    );
  });

  // Second test: Deposit 2 SOL
  it("Deposit 2 SOL", async () => {
    // Calls the `deposit` instruction with 2 SOL (in lamports)
    // Derive the vault state PDA using seeds ["state", user public key]
    const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), provider.publicKey.toBytes()], // User's public key
      program.programId
    )[0];

    // Derive the vault PDA using seeds ["vault", vaultState PDA]
    const vault = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultState.toBytes()], // Vault state PDA
      program.programId
    )[0];

    console.log("Vault State PDA:", vaultState.toBase58());
    console.log("Vault PDA:", vault.toBase58());

    const tx = await program.methods
      .deposit(new anchor.BN(2 * LAMPORTS_PER_SOL)) // Wrap 2 SOL in BN for precision. BN means BigNumber which is used to handle large numbers.
      .accountsPartial({
        user: provider.publicKey, // Signer
        vault: vault, // Vault SOL-holding account
        vaultState: vaultState, // Vault state account
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc(); // Send transaction

    // Log the signature and print vault's updated SOL balance
    console.log("\nYour transaction signature", tx);

    const vaultInfo = await provider.connection.getAccountInfo(vault);

    if (vaultInfo) {
      // If vault exists, print balance in SOL
      console.log(
        "Your vault balance",
        vaultInfo.lamports / LAMPORTS_PER_SOL,
        "SOL"
      );
    } else {
      console.log("Vault account not found");
    }
  });

  // Third test: Withdraw 1 SOL
  it("Withdraw 1 SOL", async () => {
    // Calls the `withdraw` instruction with 1 SOL
    const tx = await program.methods
      .withdraw(new anchor.BN(1 * LAMPORTS_PER_SOL)) // Withdraw 1 SOL
      .accountsPartial({
        user: provider.wallet.publicKey, // Signer
        vaultState,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc(); // Send transaction

    // Log the transaction and show raw vault data (still exists)
    console.log("\nYour transaction signature", tx);
    console.log(
      "Your vault balance",
      (await provider.connection.getAccountInfo(vault)).toString()
    );
  });

  // Final test: Close the vault and vault state
  it("Close the vault", async () => {
    // Calls the `close` instruction to close both accounts and reclaim lamports
    const tx = await program.methods
      .close()
      .accountsPartial({
        user: provider.wallet.publicKey, // Signer
        vaultState,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc(); // Send transaction

    // Log the signature and verify vault is now closed (should be null)
    console.log("\nYour transaction signature", tx);
    /*console.log("Your vault balance", 
        await provider.connection.getAccountInfo(vault)
      );*/
    const closedVaultInfo = await provider.connection.getAccountInfo(vault);

    if (closedVaultInfo) {
      // If vault exists, print balance in SOL
      console.log(
        "Your closed vault balance",
        closedVaultInfo.lamports / LAMPORTS_PER_SOL,
        "SOL"
      );
    } else {
      console.log("Vault account not found");
    }
  });
});
