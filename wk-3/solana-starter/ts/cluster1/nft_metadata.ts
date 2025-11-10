import wallet from "./wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
//import { nftStorageUploader } from "@metaplex-foundation/umi-uploader-nft-storage";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

//umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));
//umi.use(irysUploader());
//umi.use(irysUploader({ address: "https://devnet.bundlr.network" }));
// Remove the irysUploader line and use:
// Use Irys with custom timeout configuration
umi.use(
  irysUploader({
    timeout: 60000, // 60 seconds
  })
);
umi.use(signerIdentity(signer));
umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
    const image =
      "https://gateway.irys.xyz/77xD4DP3sP44bmQsBrzxQVQmYkmGDpaMnigjyyw7JxkL";
    const metadata = {
      name: "Cat wiff glass",
      symbol: "CTWG",
      description: "Gidy cat wearing glasses looking cool",
      image: image,
      attributes: [{ trait_type: "icy", value: "50" }],
      properties: {
        files: [
          {
            type: "image/jpg",
            uri: image,
          },
        ],
      },
      creators: [],
    };
    const myUri = await umi.uploader.uploadJson(metadata);
    console.log("Your metadata URI: ", myUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();

// npx tsx nft_metadata.ts
