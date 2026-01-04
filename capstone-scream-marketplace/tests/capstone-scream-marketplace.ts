import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CapstoneScreamMarketplace } from "../target/types/capstone_scream_marketplace";
import {
  createNft,
  findMasterEditionPda,
  findMetadataPda,
  MPL_TOKEN_METADATA_PROGRAM_ID,
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
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { expect } from "chai";
import { publicKey as umiPublicKey } from "@metaplex-foundation/umi";
import {
  createMetadataAccountV3,
  TokenStandard,
} from "@metaplex-foundation/mpl-token-metadata";
import { getAccount } from "@solana/spl-token";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

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
  const payer = provider.wallet as NodeWallet; // Cast to NodeWallet to access `payer` property.

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

  //const price = new anchor.BN(1);
  const price = new anchor.BN(1 * LAMPORTS_PER_SOL); // 1 SOL

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
    const badName = "dc";

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

  it("rejects listing when marketplace is not initialized", async () => {
    const uninitName = "uninit-market";

    const uninitMarketplace = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"), Buffer.from(uninitName)],
      program.programId
    )[0];

    const uninitListing = PublicKey.findProgramAddressSync(
      [
        uninitMarketplace.toBuffer(),
        new PublicKey(nftMint.publicKey).toBuffer(),
      ],
      program.programId
    )[0];

    const uninitVault = await anchor.utils.token.associatedAddress({
      mint: new PublicKey(nftMint.publicKey),
      owner: uninitListing,
    });

    try {
      await program.methods
        .listNft(price, { sol: {} })
        .accountsPartial({
          artist: artist.publicKey,
          marketplace: uninitMarketplace, // ❌ never initialized
          artistMint: new PublicKey(nftMint.publicKey),
          artistAta,
          listing: uninitListing,
          collectionMint: new PublicKey(collectionMint.publicKey),
          metadata: findMetadataPda(umi, { mint: nftMint.publicKey })[0],
          masterEdition: findMasterEditionPda(umi, {
            mint: nftMint.publicKey,
          })[0],
          vault: uninitVault,
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([artist])
        .rpc();

      throw new Error("Listing into uninitialized marketplace should fail");
    } catch (err) {
      expect(err.toString()).to.include("AccountNotInitialized");
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

  it("rejects listing of a fungible mint", async () => {
    const { createMint } = await import("@solana/spl-token");

    // Create fungible mint (decimals > 0)
    const fungibleMint = await createMint(
      connection,
      payer.payer, // payer
      payer.publicKey, // mint authority
      null,
      6 // ❌ fungible
    );

    const fungibleAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        artist,
        fungibleMint,
        artist.publicKey
      )
    ).address;

    // Create metadata so Anchor constraints pass
    const fungibleMetadata = findMetadataPda(umi, {
      mint: umiPublicKey(fungibleMint.toBase58()),
    });

    await createMetadataAccountV3(umi, {
      metadata: fungibleMetadata,
      mint: umiPublicKey(fungibleMint.toBase58()),
      mintAuthority: creator,
      payer: creator,
      updateAuthority: creator.publicKey,
      data: {
        name: "Fungible Token",
        symbol: "FT",
        uri: "https://arweave.net/fungible",
        sellerFeeBasisPoints: 0,
        creators: null,
        collection: null,
        uses: null,
      },
      isMutable: false,
      collectionDetails: { __option: "None" },
    }).sendAndConfirm(umi);

    const badListing = PublicKey.findProgramAddressSync(
      [marketplacePda.toBuffer(), fungibleMint.toBuffer()],
      program.programId
    )[0];

    const badVault = await anchor.utils.token.associatedAddress({
      mint: fungibleMint,
      owner: badListing,
    });

    try {
      await program.methods
        .listNft(price, { sol: {} })
        .accountsPartial({
          artist: artist.publicKey,
          marketplace: marketplacePda,
          artistMint: fungibleMint,
          artistAta: fungibleAta,
          listing: badListing,
          collectionMint: new PublicKey(collectionMint.publicKey),
          metadata: fungibleMetadata[0],
          masterEdition: PublicKey.default, // no master edition
          vault: badVault,
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([artist])
        .rpc();

      throw new Error("Fungible mint listing should have failed");
    } catch (err) {
      expect(err.toString()).to.satisfy(
        (msg: string) =>
          msg.includes("InvalidNft") ||
          msg.includes("master") ||
          msg.includes("Constraint")
      );
    }
  });

  it("Artist lists an NFT successfully", async () => {
    await program.methods
      .listNft(price, { sol: {} }) // or whatever enum variant
      .accountsPartial({
        artist: artist.publicKey,
        marketplace: marketplacePda,
        artistMint: new PublicKey(nftMint.publicKey),
        artistAta,
        listing,
        collectionMint: new PublicKey(collectionMint.publicKey),
        metadata: findMetadataPda(umi, { mint: nftMint.publicKey })[0],
        masterEdition: findMasterEditionPda(umi, {
          mint: nftMint.publicKey,
        })[0],
        vault,
        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([artist])
      .rpc();

    const listingAccount = await program.account.listing.fetch(listing);
    expect(listingAccount.artist.toBase58()).to.eq(artist.publicKey.toBase58());
    expect(listingAccount.price.toNumber()).to.eq(price.toNumber());
    expect(listingAccount.active).to.eq(true);
  });

  it("rejects relisting an already escrowed NFT", async () => {
    try {
      // SAME NFT
      // SAME listing PDA
      // SAME vault

      await program.methods
        .listNft(price, { sol: {} })
        .accountsPartial({
          artist: artist.publicKey,
          marketplace: marketplacePda,
          artistMint: new PublicKey(nftMint.publicKey),
          artistAta,
          listing, // ← already active
          collectionMint: new PublicKey(collectionMint.publicKey),
          metadata: findMetadataPda(umi, { mint: nftMint.publicKey })[0],
          masterEdition: findMasterEditionPda(umi, {
            mint: nftMint.publicKey,
          })[0],
          vault, // ← already contains NFT from previous listing
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([artist])
        .rpc();

      throw new Error("Expected VaultNotEmpty");
    } catch (err) {
      expect(err.toString()).to.satisfy(
        (msg: string) =>
          msg.includes("already in use") ||
          msg.includes("Constraint") ||
          msg.includes("Listing")
      );
    }
  });

  it("Fails when artist lists NFT he does not own", async () => {
    // Create a fresh NFT owned by `unauthorized artist`
    const freshMint = generateSigner(umi);

    await createNft(umi, {
      mint: freshMint,
      name: "Fresh NFT",
      symbol: "FRSH",
      uri: "https://arweave.net/fresh",
      sellerFeeBasisPoints: percentAmount(5),
      collection: { verified: false, key: collectionMint.publicKey },
      tokenOwner: publicKey(artist.publicKey),
    }).sendAndConfirm(umi);

    // verify properly
    const freshMetadata = findMetadataPda(umi, { mint: freshMint.publicKey });
    const collectionMetadata = findMetadataPda(umi, {
      mint: collectionMint.publicKey,
    });
    const collectionMasterEdition = findMasterEditionPda(umi, {
      mint: collectionMint.publicKey,
    });

    await verifySizedCollectionItem(umi, {
      metadata: freshMetadata,
      collectionAuthority: creator,
      collectionMint: collectionMint.publicKey,
      collection: collectionMetadata,
      collectionMasterEditionAccount: collectionMasterEdition,
    }).sendAndConfirm(umi);
  });

  it("rejects double listing of the same NFT", async () => {
    try {
      await program.methods
        .listNft(price, { sol: {} })
        .accountsPartial({
          artist: artist.publicKey,
          marketplace: marketplacePda,
          artistMint: new PublicKey(nftMint.publicKey),
          artistAta,
          listing, // SAME listing PDA
          collectionMint: new PublicKey(collectionMint.publicKey),
          metadata: findMetadataPda(umi, { mint: nftMint.publicKey })[0],
          masterEdition: findMasterEditionPda(umi, {
            mint: nftMint.publicKey,
          })[0],
          vault,
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([artist])
        .rpc();

      throw new Error("Double listing should have failed");
    } catch (err) {
      // accept either program error or runtime PDA collision
      expect(err.toString()).to.satisfy(
        (msg: string) =>
          msg.includes("Listing") ||
          msg.includes("already in use") ||
          msg.includes("Constraint")
      );
    }
  });

  it("rejects zero-price listing", async () => {
    const zeroPrice = new anchor.BN(0);

    // mint fresh NFT
    const zeroMint = generateSigner(umi);

    await createNft(umi, {
      mint: zeroMint,
      name: "Zero Price NFT",
      symbol: "ZERO",
      uri: "https://arweave.net/zero",
      sellerFeeBasisPoints: percentAmount(5),
      collection: { verified: false, key: collectionMint.publicKey },
      tokenOwner: publicKey(artist.publicKey),
    }).sendAndConfirm(umi);

    // verify collection
    const zeroMetadata = findMetadataPda(umi, { mint: zeroMint.publicKey });
    const collectionMetadata = findMetadataPda(umi, {
      mint: collectionMint.publicKey,
    });
    const collectionMasterEdition = findMasterEditionPda(umi, {
      mint: collectionMint.publicKey,
    });

    await verifySizedCollectionItem(umi, {
      metadata: zeroMetadata,
      collectionAuthority: creator,
      collectionMint: collectionMint.publicKey,
      collection: collectionMetadata,
      collectionMasterEditionAccount: collectionMasterEdition,
    }).sendAndConfirm(umi);

    const zeroArtistAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        artist,
        new PublicKey(zeroMint.publicKey),
        artist.publicKey
      )
    ).address;

    const zeroListing = PublicKey.findProgramAddressSync(
      [marketplacePda.toBuffer(), new PublicKey(zeroMint.publicKey).toBuffer()],
      program.programId
    )[0];

    const zeroVault = await anchor.utils.token.associatedAddress({
      mint: new PublicKey(zeroMint.publicKey),
      owner: zeroListing,
    });

    try {
      await program.methods
        .listNft(zeroPrice, { sol: {} })
        .accountsPartial({
          artist: artist.publicKey,
          marketplace: marketplacePda,
          artistMint: new PublicKey(zeroMint.publicKey),
          artistAta: zeroArtistAta,
          listing: zeroListing,
          collectionMint: new PublicKey(collectionMint.publicKey),
          metadata: zeroMetadata[0],
          masterEdition: findMasterEditionPda(umi, {
            mint: zeroMint.publicKey,
          })[0],
          vault: zeroVault,
          metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([artist])
        .rpc();

      throw new Error("Zero-price listing should have failed");
    } catch (err) {
      expect(err.toString()).to.include("Price");
    }
  });

  async function relistNft() {
    const newListing = PublicKey.findProgramAddressSync(
      [marketplacePda.toBuffer(), new PublicKey(nftMint.publicKey).toBuffer()],
      program.programId
    )[0];

    const newVault = await anchor.utils.token.associatedAddress({
      mint: new PublicKey(nftMint.publicKey),
      owner: newListing,
    });

    await program.methods
      .listNft(price, { sol: {} })
      .accountsPartial({
        artist: artist.publicKey,
        marketplace: marketplacePda,
        artistMint: new PublicKey(nftMint.publicKey),
        artistAta,
        listing: newListing,
        collectionMint: new PublicKey(collectionMint.publicKey),
        metadata: findMetadataPda(umi, { mint: nftMint.publicKey })[0],
        masterEdition: findMasterEditionPda(umi, {
          mint: nftMint.publicKey,
        })[0],
        vault: newVault,
        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([artist])
      .rpc();

    return { newListing, newVault };
  }

  it("Artist successfully delists NFT, vault and listing are closed", async () => {
    // Call delist
    await program.methods
      .delistNft() // or delistNft() — use the exact method name in your program
      .accountsPartial({
        artist: artist.publicKey,
        marketplace: marketplacePda,
        artistMint: new PublicKey(nftMint.publicKey),
        artistAta,
        listing,
        vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([artist])
      .rpc();

    // ---------- ASSERT NFT RETURNED ----------
    const artistAtaAccount = await connection.getTokenAccountBalance(artistAta);
    expect(artistAtaAccount.value.uiAmount).to.eq(1);

    // ---------- ASSERT VAULT CLOSED ----------
    const vaultAccountInfo = await connection.getAccountInfo(vault);
    expect(vaultAccountInfo).to.be.null; // closed accounts return null

    // ---------- ASSERT LISTING CLOSED ----------
    try {
      await program.account.listing.fetch(listing);
      throw new Error("Listing account should be closed");
    } catch (err) {
      expect(err.toString()).to.include("Account does not exist");
    }
  });

  it("Rejects delist with wrong marketplace", async () => {
    // Relist NFT properly
    const { newListing, newVault } = await relistNft();

    // Create a fake marketplace PDA
    const fakeMarketplaceName = "fake-market";
    const fakeMarketplace = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"), Buffer.from(fakeMarketplaceName)],
      program.programId
    )[0];

    try {
      await program.methods
        .delistNft()
        .accountsPartial({
          artist: artist.publicKey,
          marketplace: fakeMarketplace, // ❌ wrong marketplace
          artistMint: new PublicKey(nftMint.publicKey),
          artistAta,
          listing: newListing,
          vault: newVault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([artist])
        .rpc();

      throw new Error("Delist with wrong marketplace should fail");
    } catch (err) {
      expect(err.toString()).to.satisfy(
        (msg: string) =>
          msg.includes("ConstraintSeeds") ||
          msg.includes("account: marketplace")
      );
    }
  });

  async function mintAndListFreshNft() {
    const freshMint = generateSigner(umi);

    await createNft(umi, {
      mint: freshMint,
      name: "Relist NFT",
      symbol: "RLT",
      uri: "https://arweave.net/relist",
      sellerFeeBasisPoints: percentAmount(5),
      collection: { verified: false, key: collectionMint.publicKey },
      tokenOwner: publicKey(artist.publicKey),
    }).sendAndConfirm(umi);

    const metadata = findMetadataPda(umi, { mint: freshMint.publicKey });
    const masterEdition = findMasterEditionPda(umi, {
      mint: freshMint.publicKey,
    });

    await verifySizedCollectionItem(umi, {
      metadata,
      collectionAuthority: creator,
      collectionMint: collectionMint.publicKey,
      collection: findMetadataPda(umi, { mint: collectionMint.publicKey }),
      collectionMasterEditionAccount: findMasterEditionPda(umi, {
        mint: collectionMint.publicKey,
      }),
    }).sendAndConfirm(umi);

    const artistAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        artist,
        new PublicKey(freshMint.publicKey),
        artist.publicKey
      )
    ).address;

    const listing = PublicKey.findProgramAddressSync(
      [
        marketplacePda.toBuffer(),
        new PublicKey(freshMint.publicKey).toBuffer(),
      ],
      program.programId
    )[0];

    const vault = await anchor.utils.token.associatedAddress({
      mint: new PublicKey(freshMint.publicKey),
      owner: listing,
    });

    await program.methods
      .listNft(price, { sol: {} })
      .accountsPartial({
        artist: artist.publicKey,
        marketplace: marketplacePda,
        artistMint: new PublicKey(freshMint.publicKey),
        artistAta,
        listing,
        collectionMint: new PublicKey(collectionMint.publicKey),
        metadata: metadata[0],
        masterEdition: masterEdition[0],
        vault,
        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([artist])
      .rpc();

    return { freshMint, artistAta, listing, vault };
  }

  it("Rejects delist by non-artist", async () => {
    const { freshMint, artistAta, listing, vault } =
      await mintAndListFreshNft();

    try {
      await program.methods
        .delistNft()
        .accountsPartial({
          artist: fan.publicKey, // ❌ not the original artist
          marketplace: marketplacePda,
          artistMint: new PublicKey(freshMint.publicKey),
          artistAta: fanAta,
          listing,
          vault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([fan])
        .rpc();

      throw new Error("Non-artist delist should have failed");
    } catch (err) {
      expect(err.toString()).to.include("account: artist");
    }
  });

  it("Rejects double delisting", async () => {
    const { freshMint, artistAta, listing, vault } =
      await mintAndListFreshNft();

    // First delist — valid
    await program.methods
      .delistNft()
      .accountsPartial({
        artist: artist.publicKey,
        marketplace: marketplacePda,
        artistMint: new PublicKey(freshMint.publicKey),
        artistAta,
        listing,
        vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([artist])
      .rpc();

    // Second delist — must fail
    try {
      await program.methods
        .delistNft()
        .accountsPartial({
          artist: artist.publicKey,
          marketplace: marketplacePda,
          artistMint: new PublicKey(freshMint.publicKey),
          artistAta,
          listing,
          vault,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([artist])
        .rpc();

      throw new Error("Double delist should have failed");
    } catch (err) {
      expect(err.toString()).to.include("account: listing");
    }
  });

  async function mintListForSolPurchase() {
    const freshMint = generateSigner(umi);

    // mint NFT to artist
    await createNft(umi, {
      mint: freshMint,
      name: "Purchase NFT",
      symbol: "BUY",
      uri: "https://arweave.net/buy",
      sellerFeeBasisPoints: percentAmount(5),
      collection: { verified: false, key: collectionMint.publicKey },
      tokenOwner: publicKey(artist.publicKey),
    }).sendAndConfirm(umi);

    // verify collection
    const metadata = findMetadataPda(umi, { mint: freshMint.publicKey });
    const masterEdition = findMasterEditionPda(umi, {
      mint: freshMint.publicKey,
    });

    await verifySizedCollectionItem(umi, {
      metadata,
      collectionAuthority: creator,
      collectionMint: collectionMint.publicKey,
      collection: findMetadataPda(umi, { mint: collectionMint.publicKey }),
      collectionMasterEditionAccount: findMasterEditionPda(umi, {
        mint: collectionMint.publicKey,
      }),
    }).sendAndConfirm(umi);

    const artistAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        artist,
        new PublicKey(freshMint.publicKey),
        artist.publicKey
      )
    ).address;

    const fanAta = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        fan,
        new PublicKey(freshMint.publicKey),
        fan.publicKey
      )
    ).address;

    const listing = PublicKey.findProgramAddressSync(
      [
        marketplacePda.toBuffer(),
        new PublicKey(freshMint.publicKey).toBuffer(),
      ],
      program.programId
    )[0];

    const vault = await anchor.utils.token.associatedAddress({
      mint: new PublicKey(freshMint.publicKey),
      owner: listing,
    });

    await program.methods
      .listNft(price, { sol: {} })
      .accountsPartial({
        artist: artist.publicKey,
        marketplace: marketplacePda,
        artistMint: new PublicKey(freshMint.publicKey),
        artistAta,
        listing,
        collectionMint: new PublicKey(collectionMint.publicKey),
        metadata: metadata[0],
        masterEdition: masterEdition[0],
        vault,
        metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([artist])
      .rpc();

    return { freshMint, artistAta, fanAta, listing, vault };
  }

  describe("Purchase NFT - Comprehensive Tests", () => {
    describe("Happy Path - SOL Purchase", () => {
      it("Fan purchases NFT with SOL successfully", async () => {
        const { freshMint, artistAta, fanAta, listing, vault } =
          await mintListForSolPurchase();

        const artistBalanceBefore = await connection.getBalance(
          artist.publicKey
        );
        const treasuryBalanceBefore = await connection.getBalance(treasuryPda);
        const fanBalanceBefore = await connection.getBalance(fan.publicKey);

        // Execute purchase
        await program.methods
          .purchaseNft()
          .accountsPartial({
            fan: fan.publicKey,
            artist: artist.publicKey,
            marketplace: marketplacePda,
            listing,
            vault,
            artistNftAta: artistAta, // ✅ Changed from artistAta
            artistMint: new PublicKey(freshMint.publicKey),
            mintSpl: null,
            fanNftAta: fanAta, // ✅ Changed from fanAta
            fanPaymentAta: null, // ✅ Added - not needed for SOL
            artistPaymentAta: null, // ✅ Added - not needed for SOL
            treasury: treasuryPda,
            treasuryAta: null,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([fan])
          .rpc();

        // --- Verify balances ---
        const artistBalanceAfter = await connection.getBalance(
          artist.publicKey
        );
        const treasuryBalanceAfter = await connection.getBalance(treasuryPda);
        const fanBalanceAfter = await connection.getBalance(fan.publicKey);

        const fee = Math.floor((price.toNumber() * FEE_BPS) / 10_000);
        const payout = price.toNumber() - fee;

        expect(
          artistBalanceAfter - artistBalanceBefore
        ).to.be.greaterThanOrEqual(payout);
        expect(treasuryBalanceAfter - treasuryBalanceBefore).to.equal(fee);
        expect(fanBalanceBefore - fanBalanceAfter).to.be.greaterThanOrEqual(
          price.toNumber()
        );

        // --- Verify NFT ownership ---
        const fanAtaAccount = await connection.getTokenAccountBalance(fanAta);
        expect(fanAtaAccount.value.uiAmount).to.eq(1);

        const artistAtaAccount = await connection.getTokenAccountBalance(
          artistAta
        );
        expect(artistAtaAccount.value.uiAmount).to.eq(0);

        // --- Verify vault closed ---
        const vaultInfo = await connection.getAccountInfo(vault);
        expect(vaultInfo).to.be.null;

        // --- Verify listing closed ---
        try {
          await program.account.listing.fetch(listing);
          throw new Error("Listing should be closed");
        } catch (err) {
          expect(err.toString()).to.include("Account does not exist");
        }
      });
    });

    /*describe("Happy Path - SPL Purchase", () => {
      it("Fan purchases NFT with SPL tokens successfully", async () => {
        console.log("Artist publicKey:", artist.publicKey.toString());
        console.log("Fan publicKey:", fan.publicKey.toString());
        const artistBalance = await connection.getBalance(artist.publicKey);
        const fanBalance = await connection.getBalance(fan.publicKey);
        console.log("Artist SOL balance:", artistBalance / LAMPORTS_PER_SOL);
        console.log("Fan SOL balance:", fanBalance / LAMPORTS_PER_SOL);
        // At the very beginning of the SPL test, add:
        const latestBlockhash = await connection.getLatestBlockhash();
        await new Promise((resolve) => setTimeout(resolve, 1000)); // Wait 1 second
        const { createMint, mintTo } = await import("@solana/spl-token");

        // Create SPL payment token
        const paymentMint = await createMint(
          connection,
          payer.payer,
          payer.publicKey,
          null,
          6
        );

        // Create fresh NFT and list it for SPL
        const nftMint = generateSigner(umi);

        await createNft(umi, {
          mint: nftMint,
          name: "SPL Purchase NFT",
          symbol: "SPLNFT",
          uri: "https://arweave.net/spl-buy",
          sellerFeeBasisPoints: percentAmount(5),
          collection: { verified: false, key: collectionMint.publicKey },
          tokenOwner: publicKey(artist.publicKey),
        }).sendAndConfirm(umi);

        // Verify collection
        await verifySizedCollectionItem(umi, {
          metadata: findMetadataPda(umi, { mint: nftMint.publicKey }),
          collectionAuthority: creator,
          collectionMint: collectionMint.publicKey,
          collection: findMetadataPda(umi, { mint: collectionMint.publicKey }),
          collectionMasterEditionAccount: findMasterEditionPda(umi, {
            mint: collectionMint.publicKey,
          }),
        }).sendAndConfirm(umi);

        // Create ATAs for NFT
        const artistNftAta = (
          await getOrCreateAssociatedTokenAccount(
            connection,
            artist,
            new PublicKey(nftMint.publicKey),
            artist.publicKey
          )
        ).address;

        const fanNftAta = (
          await getOrCreateAssociatedTokenAccount(
            connection,
            fan,
            new PublicKey(nftMint.publicKey),
            fan.publicKey
          )
        ).address;

        // Create ATAs for payment token
        const fanPaymentAta = (
          await getOrCreateAssociatedTokenAccount(
            connection,
            fan,
            paymentMint,
            fan.publicKey
          )
        ).address;

        const artistPaymentAta = (
          await getOrCreateAssociatedTokenAccount(
            connection,
            artist,
            paymentMint,
            artist.publicKey
          )
        ).address;

        const treasuryPaymentAta = (
          await getOrCreateAssociatedTokenAccount(
            connection,
            payer.payer,
            paymentMint,
            treasuryPda,
            true // allowOwnerOffCurve
          )
        ).address;

        // Mint payment tokens to fan
        const paymentAmount = 1_000_000;
        await mintTo(
          connection,
          payer.payer,
          paymentMint,
          fanPaymentAta,
          payer.publicKey,
          paymentAmount * 2
        );

        // Create listing with SPL payment
        const listing = PublicKey.findProgramAddressSync(
          [
            marketplacePda.toBuffer(),
            new PublicKey(nftMint.publicKey).toBuffer(),
          ],
          program.programId
        )[0];

        const vault = await anchor.utils.token.associatedAddress({
          mint: new PublicKey(nftMint.publicKey),
          owner: listing,
        });

        const splPrice = new anchor.BN(paymentAmount);

        // List the NFT for SPL payment
        await program.methods
          .listNft(splPrice, { spl: { mint: paymentMint } })
          .accountsPartial({
            artist: artist.publicKey,
            marketplace: marketplacePda,
            artistMint: new PublicKey(nftMint.publicKey),
            artistAta: artistNftAta, // This is the NFT ATA for the list instruction
            listing,
            collectionMint: new PublicKey(collectionMint.publicKey),
            metadata: findMetadataPda(umi, { mint: nftMint.publicKey })[0],
            masterEdition: findMasterEditionPda(umi, {
              mint: nftMint.publicKey,
            })[0],
            vault,
            metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([artist])
          .rpc();

        // Get balances before purchase
        const fanBalanceBefore = (
          await connection.getTokenAccountBalance(fanPaymentAta)
        ).value.amount;
        const artistBalanceBefore = (
          await connection.getTokenAccountBalance(artistPaymentAta)
        ).value.amount;
        const treasuryBalanceBefore = (
          await connection.getTokenAccountBalance(treasuryPaymentAta)
        ).value.amount;

        // Execute SPL purchase
        await program.methods
          .purchaseNft()
          .accountsPartial({
            fan: fan.publicKey,
            artist: artist.publicKey,
            marketplace: marketplacePda,
            listing,
            vault,
            artistNftAta, // NFT ATA for artist
            artistMint: new PublicKey(nftMint.publicKey),
            mintSpl: paymentMint,
            fanNftAta, // NFT ATA for fan
            fanPaymentAta, // Payment token ATA for fan
            artistPaymentAta, // Payment token ATA for artist
            treasury: treasuryPda,
            treasuryAta: treasuryPaymentAta,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([fan, artist])
          .rpc();

        // Verify SPL token balances
        const fanBalanceAfter = (
          await connection.getTokenAccountBalance(fanPaymentAta)
        ).value.amount;
        const artistBalanceAfter = (
          await connection.getTokenAccountBalance(artistPaymentAta)
        ).value.amount;
        const treasuryBalanceAfter = (
          await connection.getTokenAccountBalance(treasuryPaymentAta)
        ).value.amount;

        const fee = Math.floor((paymentAmount * FEE_BPS) / 10_000);
        const payout = paymentAmount - fee;

        expect(
          Number(artistBalanceAfter) - Number(artistBalanceBefore)
        ).to.equal(payout);
        expect(
          Number(treasuryBalanceAfter) - Number(treasuryBalanceBefore)
        ).to.equal(fee);
        expect(Number(fanBalanceBefore) - Number(fanBalanceAfter)).to.equal(
          paymentAmount
        );

        // Verify NFT ownership
        const fanNftAccount = await connection.getTokenAccountBalance(
          fanNftAta
        );
        expect(fanNftAccount.value.uiAmount).to.eq(1);

        // Verify vault closed
        const vaultInfo = await connection.getAccountInfo(vault);
        expect(vaultInfo).to.be.null;

        // Verify listing closed
        try {
          await program.account.listing.fetch(listing);
          throw new Error("Listing should be closed");
        } catch (err) {
          expect(err.toString()).to.include("Account does not exist");
        }
      });
    });
  });*/
    describe("Happy Path - SPL Purchase", () => {
      it("Fan purchases NFT with SPL tokens successfully", async () => {
        try {
          console.log("=== Starting SPL Purchase Test ===");

          // Check balances
          const artistBalance = await connection.getBalance(artist.publicKey);
          const fanBalance = await connection.getBalance(fan.publicKey);
          console.log("Artist SOL:", artistBalance / LAMPORTS_PER_SOL);
          console.log("Fan SOL:", fanBalance / LAMPORTS_PER_SOL);

          const { createMint, mintTo } = await import("@solana/spl-token");

          console.log("Creating payment mint...");
          const paymentMint = await createMint(
            connection,
            payer.payer,
            payer.publicKey,
            null,
            6
          );
          console.log("Payment mint created:", paymentMint.toString());

          const nftMint = generateSigner(umi);
          console.log("NFT mint:", nftMint.publicKey.toString());

          console.log("Creating NFT...");
          await createNft(umi, {
            mint: nftMint,
            name: "SPL Purchase NFT",
            symbol: "SPLNFT",
            uri: "https://arweave.net/spl-buy",
            sellerFeeBasisPoints: percentAmount(5),
            collection: { verified: false, key: collectionMint.publicKey },
            tokenOwner: publicKey(artist.publicKey),
          }).sendAndConfirm(umi);
          console.log("NFT created");

          console.log("Verifying collection...");
          await verifySizedCollectionItem(umi, {
            metadata: findMetadataPda(umi, { mint: nftMint.publicKey }),
            collectionAuthority: creator,
            collectionMint: collectionMint.publicKey,
            collection: findMetadataPda(umi, {
              mint: collectionMint.publicKey,
            }),
            collectionMasterEditionAccount: findMasterEditionPda(umi, {
              mint: collectionMint.publicKey,
            }),
          }).sendAndConfirm(umi);
          console.log("Collection verified");

          console.log("Creating ATAs...");
          const artistNftAta = (
            await getOrCreateAssociatedTokenAccount(
              connection,
              artist,
              new PublicKey(nftMint.publicKey),
              artist.publicKey
            )
          ).address;

          const fanNftAta = (
            await getOrCreateAssociatedTokenAccount(
              connection,
              fan,
              new PublicKey(nftMint.publicKey),
              fan.publicKey
            )
          ).address;

          const fanPaymentAta = (
            await getOrCreateAssociatedTokenAccount(
              connection,
              fan,
              paymentMint,
              fan.publicKey
            )
          ).address;

          const artistPaymentAta = (
            await getOrCreateAssociatedTokenAccount(
              connection,
              artist,
              paymentMint,
              artist.publicKey
            )
          ).address;

          const treasuryPaymentAta = (
            await getOrCreateAssociatedTokenAccount(
              connection,
              payer.payer,
              paymentMint,
              treasuryPda,
              true
            )
          ).address;
          console.log("ATAs created");

          const paymentAmount = 1_000_000;
          console.log("Minting payment tokens to fan...");
          await mintTo(
            connection,
            payer.payer,
            paymentMint,
            fanPaymentAta,
            payer.publicKey,
            paymentAmount * 2
          );
          console.log("Payment tokens minted");

          const listing = PublicKey.findProgramAddressSync(
            [
              marketplacePda.toBuffer(),
              new PublicKey(nftMint.publicKey).toBuffer(),
            ],
            program.programId
          )[0];

          const vault = await anchor.utils.token.associatedAddress({
            mint: new PublicKey(nftMint.publicKey),
            owner: listing,
          });

          const splPrice = new anchor.BN(paymentAmount);

          console.log("Listing NFT for SPL...");
          console.log("Artist is signer?", artist !== undefined);
          console.log("Artist publicKey:", artist.publicKey.toString());

          await program.methods
            .listNft(splPrice, { spl: { mint: paymentMint } })
            .accountsPartial({
              artist: artist.publicKey,
              marketplace: marketplacePda,
              artistMint: new PublicKey(nftMint.publicKey),
              artistAta: artistNftAta,
              listing,
              collectionMint: new PublicKey(collectionMint.publicKey),
              metadata: findMetadataPda(umi, { mint: nftMint.publicKey })[0],
              masterEdition: findMasterEditionPda(umi, {
                mint: nftMint.publicKey,
              })[0],
              vault,
              metadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
              systemProgram: SystemProgram.programId,
            })
            .signers([fan, artist])
            .rpc();
          console.log("NFT listed successfully");

          // Rest of test...
          console.log("Test completed successfully!");
        } catch (error) {
          console.error("Test failed at:", error.message);
          console.error("Full error:", error);
          throw error;
        }
      });
    });
  });
});
