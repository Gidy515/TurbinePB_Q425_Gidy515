use anchor_lang::prelude::*;

declare_id!("2d8HV2bbi5j59yxnnfsPphvQhGQHHbzyPasbDBLbkAmo");

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod error;
pub use error::*;

#[program]
pub mod capstone_scream_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, ctx.bumps)?;
        //msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn list_nft(ctx: Context<List>, price: u64, payment_currency: PaymentCurrency) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps, payment_currency)?;
        ctx.accounts.deposit_nft()?;

        Ok(())
    }

    pub fn delist_nft(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_mint_vault()?;
        Ok(())
    }

    pub fn purchase_nft(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.purchase()?;
        Ok(())
    }
}

/*describe("Happy Path - SPL Purchase", () => {
      it("Fan purchases NFT with SPL tokens successfully", async () => {
        const { createMint, mintTo } = await import("@solana/spl-token");

        const paymentMint = await createMint(
          connection,
          payer.payer,
          payer.publicKey,
          null,
          6
        );

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

        await verifySizedCollectionItem(umi, {
          metadata: findMetadataPda(umi, { mint: nftMint.publicKey }),
          collectionAuthority: creator,
          collectionMint: collectionMint.publicKey,
          collection: findMetadataPda(umi, { mint: collectionMint.publicKey }),
          collectionMasterEditionAccount: findMasterEditionPda(umi, {
            mint: collectionMint.publicKey,
          }),
        }).sendAndConfirm(umi);

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

        const paymentAmount = 1_000_000;
        await mintTo(
          connection,
          payer.payer,
          paymentMint,
          fanPaymentAta,
          payer.publicKey,
          paymentAmount * 2
        );

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
          .signers([artist])
          .rpc();

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
            artistNftAta, // ✅ NFT ATA
            artistMint: new PublicKey(nftMint.publicKey),
            mintSpl: paymentMint,
            fanNftAta, // ✅ NFT ATA
            fanPaymentAta, // ✅ Payment ATA
            artistPaymentAta, // ✅ Payment ATA
            treasury: treasuryPda,
            treasuryAta: treasuryPaymentAta,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([fan])
          .rpc();

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

        const fanNftAccount = await connection.getTokenAccountBalance(
          fanNftAta
        );
        expect(fanNftAccount.value.uiAmount).to.eq(1);

        const vaultInfo = await connection.getAccountInfo(vault);
        expect(vaultInfo).to.be.null;

        try {
          await program.account.listing.fetch(listing);
          throw new Error("Listing should be closed");
        } catch (err) {
          expect(err.toString()).to.include("Account does not exist");
        }
      });
    });
  });*/
















