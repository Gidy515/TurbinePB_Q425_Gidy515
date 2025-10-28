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

(async () => {})();
