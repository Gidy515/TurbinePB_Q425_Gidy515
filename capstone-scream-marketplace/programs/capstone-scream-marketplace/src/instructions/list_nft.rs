use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, 
    metadata::{MasterEditionAccount, Metadata, MetadataAccount}, 
    token::{transfer_checked, TransferChecked}, 
    token_interface::{Mint, TokenAccount, TokenInterface}
};

use crate::{PaymentCurrency, state::{Listing, Marketplace}};
use crate::error::MarketplaceError;

#[derive(Accounts)]
pub struct List <'info> {
    #[account(mut)]
    pub artist: Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub artist_mint: InterfaceAccount<'info, Mint>, // The NFT mint being listed

    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = artist,
    )]
    pub artist_ata: InterfaceAccount<'info, TokenAccount>, // The artist's token account that holds the NFT

    #[account(
        init,
        payer = artist,
        seeds = [marketplace.key().as_ref(), artist_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE,
    )]
    pub listing: Account<'info, Listing>, // Account to store listing information

    pub collection_mint: InterfaceAccount<'info, Mint>, 

    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), artist_mint.key().as_ref(),],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>, // NFT metadata to verify collection
     
    #[account(
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(),
            artist_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>, // Master edition to verify it's an NFT

    pub metadata_program: Program<'info, Metadata>, // Metaplex program

    #[account(
        init_if_needed,
        payer = artist,
        associated_token::mint = artist_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>, // Escrow account for the NFT during listing

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>, 
    pub system_program: Program<'info, System>,  
}

impl <'info> List <'info> {
    pub fn create_listing(&mut self, price: u64, bumps: &ListBumps, payment_currency: PaymentCurrency) ->Result<()>{
        // Price must be greater than zero to prevent free listings
        require!(price > 0, MarketplaceError::InvalidPrice);
        
        // Ensure marketplace is properly initialized
        require!(
            self.marketplace.admin != Pubkey::default(),
            MarketplaceError::InvalidMarketplaceState
        );

        // Ensure the NFT is not already listed    
        require!(
            self.listing.artist == Pubkey::default(),
            MarketplaceError::AlreadyListed
        );
        
        // Ensure the artist owns the NFT
        require!(
            self.artist_ata.amount == 1,
            MarketplaceError::InvalidNftOwnership
        );

        // Ensure the mint is non-fungible (NFT) 
        require!(
            self.artist_mint.decimals == 0,
            MarketplaceError::InvalidMintDecimals
        );
        require!(
            self.artist_mint.supply == 1,
            MarketplaceError::InvalidMintSupply
        );
        require!(
            self.artist_ata.amount == 1,
            MarketplaceError::InvalidTokenAmount
        );

        self.listing.set_inner(Listing{
            artist: self.artist.key(),
            artist_mint: self.artist_mint.key(),
            price,
            bump: bumps.listing,
            marketplace: self.marketplace.key(),
            payment_currency,
            active: true,
        });

        Ok(())
    }
    
    pub fn deposit_nft(&mut self) ->Result<()>{
            // Ensure the vault is empty before depositing the NFT
        require!(
            self.vault.amount == 0,
            MarketplaceError::VaultNotEmpty
        );

        // Ensure the artist still owns the NFT before depositing
        require!(
            self.artist_ata.amount == 1,
            MarketplaceError::InvalidTokenOwner
        );
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from: self.artist_ata.to_account_info(), 
            mint: self.artist_mint.to_account_info(),  
            to: self.vault.to_account_info(), 
            authority: self.artist.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(
            cpi_ctx, self.artist_ata.amount, 
            self.artist_mint.decimals
        ).map_err(|_| MarketplaceError::NftTransferFailed)?;

        Ok(())
    }
}