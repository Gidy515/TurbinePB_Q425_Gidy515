import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CapstoneScreamMarketplace } from "../target/types/capstone_scream_marketplace";
import {
  createNft,
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
  verifySizedCollectionItem,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  KeypairSigner,
  createSignerFromKeypair,
  generateSigner,
  keypairIdentity,
  percentAmount,
  publicKey,
} from "@metaplex-foundation/umi";
import {
  TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { expect } from "chai";

describe("capstone-scream-marketplace", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .capstoneScreamMarketplace as Program<CapstoneScreamMarketplace>;

  const admin = provider.wallet;

  const MARKETPLACE_NAME = "scream-market";
  const FEE_BPS = 250; // 2.5%

  const connection = provider.connection;
  const umi = createUmi(provider.connection);
  const payer = provider.wallet as NodeWallet;

  let nftMint: KeypairSigner = generateSigner(umi);
  let collectionMint: KeypairSigner = generateSigner(umi);

  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(
    new Uint8Array(payer.payer.secretKey)
  );

  const creator = createSignerFromKeypair(umi, creatorWallet);
  umi.use(keypairIdentity(creator));
  umi.use(mplTokenMetadata());

  let artistAta: anchor.web3.PublicKey;
  let fanAta: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;

  const artist = Keypair.generate();
  const fan = Keypair.generate();

  const price = new anchor.BN(1);

  let marketplacePda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("marketplace"), Buffer.from(MARKETPLACE_NAME)],
    program.programId
  )[0];

  /*const rewardsMint = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("rewards"), marketplace.toBuffer()],
    program.programId
  )[0];*/

  let treasuryPda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury"), marketplacePda.toBuffer()],
    program.programId
  )[0];

  const listing = anchor.web3.PublicKey.findProgramAddressSync(
    [
      marketplacePda.toBuffer(),
      new anchor.web3.PublicKey(
        nftMint.publicKey as unknown as PublicKey
      ).toBuffer(),
    ],
    program.programId
  )[0];

  before(async () => {
    // Setup code can be added here if needed.
    // Airdrop SOL to artist and fan
    const artistAirdrop = await connection.requestAirdrop(
      artist.publicKey,
      7 * LAMPORTS_PER_SOL
    );
    const fanAirdrop = await connection.requestAirdrop(
      fan.publicKey,
      7 * LAMPORTS_PER_SOL
    );
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature: artistAirdrop,
      ...latestBlockhash,
    });
    await connection.confirmTransaction({
      signature: fanAirdrop,
      ...latestBlockhash,
    });
    //await sleep(2000);

    // Mint Collection NFT
    await createNft(umi, {
      mint: collectionMint,
      name: "Calm down",
      symbol: "CMD",
      uri: "https://arweave.net/123",
      sellerFeeBasisPoints: percentAmount(5.5),
      collectionDetails: { __kind: "V1", size: 10 },
    }).sendAndConfirm(umi);
    console.log(
      `Created Collection NFT: ${collectionMint.publicKey.toString()}`
    );

    // Mint NFT into the artist's ATA
    await createNft(umi, {
      mint: nftMint,
      name: "Calm down",
      symbol: "CMD",
      uri: "https://arweave.net/123",
      sellerFeeBasisPoints: percentAmount(5.5),
      collection: { verified: false, key: collectionMint.publicKey },
      tokenOwner: publicKey(artist.publicKey), // Corrected to use artist's public key
    }).sendAndConfirm(umi);
    console.log(`Created NFT: ${nftMint.publicKey.toString()}`);

    // Verify Collection
    const collectionMetadata = findMetadataPda(umi, {
      mint: collectionMint.publicKey,
    });

    const collectionMasterEdition = findMasterEditionPda(umi, {
      mint: collectionMint.publicKey,
    });

    const nftMetadata = findMetadataPda(umi, { mint: nftMint.publicKey });

    await verifySizedCollectionItem(umi, {
      metadata: nftMetadata,
      collectionAuthority: creator,
      collectionMint: collectionMint.publicKey,
      collection: collectionMetadata,
      collectionMasterEditionAccount: collectionMasterEdition,
    }).sendAndConfirm(umi);
    console.log("Collection NFT Verified!");

    // Get or create ATAs
    artistAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        artist,
        new anchor.web3.PublicKey(nftMint.publicKey),
        artist.publicKey
      )
    ).address;

    fanAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        fan,
        new anchor.web3.PublicKey(nftMint.publicKey),
        fan.publicKey
      )
    ).address;

    vault = await anchor.utils.token.associatedAddress({
      mint: new anchor.web3.PublicKey(nftMint.publicKey),
      owner: listing,
    });
  });

  it("Marketplace initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(MARKETPLACE_NAME, FEE_BPS)
      .accountsPartial({
        admin: admin.publicKey,
        marketplace: marketplacePda,
        treasury: treasuryPda,
        treasuryPaymentAta: null, // optional account
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Marketplace initialization signature", tx);
  });

  it("rejects marketplace name that is too short", async () => {
    const badName = "ab";

    const badMarketplacePda = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"), Buffer.from(badName)],
      program.programId
    )[0];

    const badTreasuryPda = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), badMarketplacePda.toBuffer()],
      program.programId
    )[0];

    try {
      await program.methods
        .initialize(badName, FEE_BPS)
        .accountsPartial({
          admin: admin.publicKey,
          marketplace: badMarketplacePda,
          treasury: badTreasuryPda,
          treasuryPaymentAta: null,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      throw new Error("Short name should have failed");
    } catch (err) {
      expect(err.toString()).to.include("NameTooShort");
    }
  });

  it("rejects marketplace name with invalid characters", async () => {
    const badName = "scream@market!";

    const badMarketplacePda = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"), Buffer.from(badName)],
      program.programId
    )[0];

    const badTreasuryPda = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), badMarketplacePda.toBuffer()],
      program.programId
    )[0];

    try {
      await program.methods
        .initialize(badName, FEE_BPS)
        .accountsPartial({
          admin: admin.publicKey,
          marketplace: badMarketplacePda,
          treasury: badTreasuryPda,
          treasuryPaymentAta: null,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      throw new Error("Invalid characters should have failed");
    } catch (err) {
      expect(err.toString()).to.include("InvalidMarketplaceName");
    }
  });

  it("rejects excessive marketplace fee", async () => {
    const highFee = 5_000; // 50%

    const name = "high-fee-market";

    const badMarketplacePda = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"), Buffer.from(name)],
      program.programId
    )[0];

    const badTreasuryPda = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"), badMarketplacePda.toBuffer()],
      program.programId
    )[0];

    try {
      await program.methods
        .initialize(name, highFee)
        .accountsPartial({
          admin: admin.publicKey,
          marketplace: badMarketplacePda,
          treasury: badTreasuryPda,
          treasuryPaymentAta: null,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      throw new Error("High fee should have failed");
    } catch (err) {
      expect(err.toString()).to.include("InvalidFee");
    }
  });

  it("rejects incorrect treasury PDA address (account)", async () => {
    const name = "bad-treasury-market";

    const marketplacePda = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"), Buffer.from(name)],
      program.programId
    )[0];

    const fakeTreasury = Keypair.generate().publicKey;

    try {
      await program.methods
        .initialize(name, FEE_BPS)
        .accountsPartial({
          admin: admin.publicKey,
          marketplace: marketplacePda,
          treasury: fakeTreasury,
          treasuryPaymentAta: null,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      throw new Error("Wrong treasury PDA should have failed");
    } catch (err) {
      expect(err.toString()).to.include("ConstraintSeeds");
    }
  });

  it("rejects re-initialization of the same marketplace", async () => {
    try {
      await program.methods
        .initialize(MARKETPLACE_NAME, FEE_BPS)
        .accountsPartial({
          admin: admin.publicKey,
          marketplace: marketplacePda,
          treasury: treasuryPda,
          treasuryPaymentAta: null,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      throw new Error("Re-initialization should have failed");
    } catch (err) {
      // This fails at runtime level (account already in use)
      expect(err.toString()).to.include("already in use");
    }
  });
});
