use crate::state::{Listing, Marketplace};
use anchor_lang::prelude::*;
use anchor_spl::{token::{close_account, transfer_checked, CloseAccount, TransferChecked}, 
    token_interface::{Mint, TokenAccount, TokenInterface}, 
    associated_token::AssociatedToken
,};

use crate::error::MarketplaceError;

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub artist: Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    artist_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::authority = artist,
        associated_token::mint = artist_mint,
    )]
    pub artist_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = artist,
        seeds = [marketplace.key().as_ref(), artist_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) ->Result<()>{

        // Only the original artist can delist the NFT
        require!(
            self.listing.artist == self.artist.key(),
            MarketplaceError::UnauthorizedDelist
        );

        // Ensure the listing belongs to the correct marketplace
        require!(
            self.listing.marketplace == self.marketplace.key(),
            MarketplaceError::InvalidMarketplace
        );

        // Ensure the mint matches the listing
        require!(
            self.listing.artist_mint == self.artist_mint.key(),
            MarketplaceError::InvalidArtistMint
        );

        // Validate that the mint is indeed an NFT
        require!(
            self.artist_mint.decimals == 0,
            MarketplaceError::InvalidMintDecimals
        );
        require!(
            self.artist_mint.supply == 1,
            MarketplaceError::InvalidMintSupply
        );

        // Ensure the vault holds exactly one token (the NFT)
        require!(
            self.vault.amount == 1,
            MarketplaceError::InvalidVaultAmount
        );

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),  
            mint: self.artist_mint.to_account_info(), 
            to: self.artist_ata.to_account_info(), 
            authority: self.listing.to_account_info(), 
        };

        // Create signer seeds for the listing PDA to authorize the transfer
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.artist_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Transfer the NFT back to the original owner
        transfer_checked(cpi_ctx, self.vault.amount, self.artist_mint.decimals)?;

        Ok(())
    }

    pub fn close_mint_vault(&mut self)->Result<()>{
        // Ensure the vault is empty before closing
        require!(
            self.vault.amount == 0,
            MarketplaceError::VaultNotEmpty
        );

        let seeds = &[
            &self.marketplace.key().to_bytes()[..], 
            &self.artist_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(), // The vault account to close
            destination: self.artist.to_account_info(), // Return rent to maker
            authority: self.listing.to_account_info(), // Auth: listing PDA
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Close the vault and return the rent to the maker
        close_account(cpi_ctx).map_err(|_| MarketplaceError::VaultCloseFailed)?;

        Ok(())
    }
}   