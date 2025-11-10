import {
  Commitment,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import wallet from "./wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("J3YbfzYE8ShsPU8Rb4LZe3c1zuRBXA2dDTrTm3vcF4Vj");

// Recipient address
const to = new PublicKey("DvDVaELzjP9imm2KdyikVtp9qHNnk3moTHksefGh6vTf");

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const fromWalletAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );
    // Get the token account of the toWallet address, and if it does not exist, create it
    const toWalletAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );
    // Transfer the new token to the "toTokenAccount" we just created
    const txsig = await transfer(
      connection,
      keypair,
      fromWalletAta.address,
      toWalletAta.address,
      keypair.publicKey,
      10e6 // 10 tokens, remember 6 decimals
    );
    console.log(`Transfer txid: ${txsig}`);
    // 1e6 = 1 token with 6 decimals
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
