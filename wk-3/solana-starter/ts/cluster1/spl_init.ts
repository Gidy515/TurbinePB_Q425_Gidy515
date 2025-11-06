import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import wallet from "./wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
  try {
    // Start here
    const mintID = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      null,
      6 // Number of decimal places in the token
    );
    console.log(`Mint Address: ${mintID.toBase58()}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();

// Mint Address: 5SQmkV3KrAXX3KACStFELGCL6iDSoMcT6njTQxW6eu32
