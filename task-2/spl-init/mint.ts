import { Keypair, Connection, Commitment, PublicKey } from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import wallet from "./wallet.json" assert { type: "json" };

// import keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
// define commitment level
const commitment: Commitment = "confirmed";
// create connection to the Solana devnet
const connection = new Connection("https://api.devnet.solana.com", commitment);

const myWalletPublicKey = keypair.publicKey;

const token_decimals = 1_000_000; // 1 token with 6 decimals

const mint = new PublicKey("6CYENncbX7Nnn8U2jezP1CtbL9Ymbfc9bPGmdUgVRoFp");

(async () => {
  try {
    // Get or create the associated token account for the wallet and mint
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair, // payer
      mint, // mint
      myWalletPublicKey // owner
    );
    console.log("Associated Token Account:", ata.address.toBase58());

    // Mint to ATA
    /*const mintTx = await mintTo(
      connection,
      keypair, // payer
      mint, // mint
      ata.address, // destination
      myWalletPublicKey, // authority
      10 * token_decimals // amount
    );*/
    //console.log("Mint Signature:", mintSignature);
  } catch (error) {
    console.error("Error minting tokens:", error);
  }
})();

// ATA: 6rWRZMHRHsNYqV3qqC7jDRFBMXbgX5wZ7nfTgmR31aCf
