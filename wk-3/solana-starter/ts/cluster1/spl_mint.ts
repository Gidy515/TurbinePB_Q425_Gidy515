import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import wallet from "./wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

//const token_decimals = 1_000_000n;

// Token details
const decimals = 6; // ✅ your token has 6 decimals
const tokensToMint = 100; // 100 tokens
const amountToMint = BigInt(tokensToMint * 10 ** decimals); // 100 * 1,000,000 = 100,000,000

const owner = keypair.publicKey;

// Mint address
const mint = new PublicKey("J3YbfzYE8ShsPU8Rb4LZe3c1zuRBXA2dDTrTm3vcF4Vj");

(async () => {
  try {
    // Create an ATA
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      owner,
      true,
      commitment
    );
    console.log(`Your ata is: ${ata.address.toBase58()}`);
    // Mint to ATA
    const mintTx = await mintTo(
      connection,
      keypair,
      mint,
      ata.address,
      keypair,
      amountToMint
    );
    console.log(`Your mint txid, minted 100 Gds tokens: ${mintTx}`);
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();

// ATA: 3neKUiqXV2X6D9PCBQAE1w61xfTHSTUpkUXDDXk6pyhe
