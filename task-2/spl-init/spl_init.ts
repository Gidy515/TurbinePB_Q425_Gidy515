import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import wallet from "./wallet.json" assert { type: "json" };

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const commitment: Commitment = "confirmed";

const connection = new Connection("https://api.devnet.solana.com", commitment);

const myWalletPublicKey = keypair.publicKey;

(async () => {
  try {
    const mint = await createMint(
      connection, // Connection
      keypair, // Payer
      myWalletPublicKey, // Mint Authority
      null, // Freeze Authority
      6 // Decimals (this figure is used mostly for stablecoins like USDC)
    );
    console.log("Mint Address:", mint.toBase58());
  } catch (error) {
    console.error("Error creating mint:", error);
  }
})();

// Mint Address: 6CYENncbX7Nnn8U2jezP1CtbL9Ymbfc9bPGmdUgVRoFp

// Mint {
// Optional authority used to mint new tokens. The mint authority may only be provided during mint creation.
// If no mint authoriy is present then the mint has a fixed supply and no new tokens may be minted.
// pub mintAuthority: COption<Pubkey>,
/// Total supply of tokens.
// pub supply: u64,
/// Number of base 10 digits to the right of the decimal place.
// pub decimals: u8,
/// Is `true` if this struct has been initialized.
// pub isInitialized: bool,
/// Optional authority to freeze token accounts associated with this mint.
// pub freezeAuthority: COption<Pubkey>,
//}
