import {
  Connection,
  PublicKey,
  Keypair,
  Commitment,
  SystemProgram,
} from "@solana/web3.js";
import wallet from "./wallet.json" assert { type: "json" };
import {
  CreateMetadataAccountV3InstructionData,
  PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID,
  DataV2,
  createMetadataAccountV3,
  getCreateMetadataAccountV3InstructionDataSerializer,
} from "@metaplex-foundation/mpl-token-metadata";
import { sendAndConfirmTransaction, Transaction } from "@solana/web3.js";

// Load your keypair
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Connection setup
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Replace with your actual mint address
const mint = new PublicKey("6CYENncbX7Nnn8U2jezP1CtbL9Ymbfc9bPGmdUgVRoFp");

// Derive the PDA (Program Derived Address) for the Metadata account
async function getMetadataPDA(mint: PublicKey): Promise<PublicKey> {
  const [metadataPDA] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );
  return metadataPDA;
}

(async () => {
  try {
    const metadataPDA = await getMetadataPDA(mint);

    // Create metadata content
    const dataV2: DataV2 = {
      name: "Gidy Token", // 🪙 Token Name
      symbol: "GIDY", // 💡 Symbol
      uri: "https://gateway.pinata.cloud/ipfs/QmExampleMetadataJSON", // Link to JSON (example below)
      sellerFeeBasisPoints: 0,
      creators: {
        __option: "None",
      },
      collection: {
        __option: "None",
      },
      uses: {
        __option: "None",
      },
    };

    // Instruction to create metadata
    const ix = getCreateMetadataAccountV3InstructionDataSerializer({
      createMetadataAccountArgsV3: {
        data: dataV2,
        isMutable: true,
        collectionDetails: null,
      },
    }).serialize({
      metadata: metadataPDA,
      mint: mint,
      mintAuthority: keypair.publicKey,
      payer: keypair.publicKey,
      updateAuthority: keypair.publicKey,
    });
    // Create and send transaction
    const tx = new Transaction().add(ix);

    const txSig = await sendAndConfirmTransaction(connection, tx, [keypair]);
    console.log("✅ Metadata created successfully!");
    console.log("Metadata Account PDA:", metadataPDA.toBase58());
    console.log("Transaction Signature:", txSig);
  } catch (err) {
    console.error("❌ Error creating metadata:", err);
  }
})();
