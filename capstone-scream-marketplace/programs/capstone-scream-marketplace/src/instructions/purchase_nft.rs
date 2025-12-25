use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{state::Marketplace, Listing, MarketplaceError, PaymentCurrency};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub fan: Signer<'info>, // buyer

    #[account(mut)]
    pub artist: SystemAccount<'info>, // seller

    pub artist_mint: InterfaceAccount<'info, Mint>, // NFT mint

    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = artist,
        associated_token::token_program = token_program,
    )]
    pub artist_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        init_if_needed,
        payer = fan,
        associated_token::mint = artist_mint,
        associated_token::authority = fan
    )]
    pub fan_ata: InterfaceAccount<'info, TokenAccount>, // buyer ATA

    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>, // escrow holding NFT

    #[account(
        mut,
        seeds = [artist_mint.key().as_ref(), marketplace.key().as_ref()],
        bump = listing.bump,
        close = artist
    )]
    pub listing: Account<'info, Listing>, // listing PDA

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>, // marketplace fee vault

    // SPL treasury ATA (only used if SPL)
    #[account(
        mut,
        associated_token::mint = mint_spl,
        associated_token::authority = treasury,
        associated_token::token_program = token_program,
    )]
    pub treasury_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_mint_bump,
        mint::authority = marketplace,
        mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_spl: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


impl <'info> Purchase<'info>{
    pub fn pay_in_sol(&mut self) ->Result<()>{
        
        /*let cpi_program = self.system_program.to_account_info();

        let cpi_accounts= Transfer{
            from: self.fan.to_account_info(), // From buyer
            to: self.artist.to_account_info(), // To seller
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);*/
        // Validate listing
        require!(self.listing.marketplace == self.marketplace.key(), MarketplaceError::InvalidMarketplaceListing); // Ensure the marketplace matches the listing
        require!(self.listing.artist == self.artist.key(), MarketplaceError::WrongArtist); // Ensure the artist is the original lister
        require!(self.listing.artist_mint == self.artist_mint.key(), MarketplaceError::WrongTokenMint); // Ensure the mint matches the listing

        // Validate that the vault holds the NFT
        require!(self.vault.amount == 1, MarketplaceError::WrongVaultAmount); // Ensure the vault holds exactly one token (the NFT) to prevent double spending
        require!(self.artist_mint.supply == 1, MarketplaceError::WrongMintSupply); // Validate that the mint is indeed an NFT
        require!(self.artist_mint.decimals == 0, MarketplaceError::WrongMintDecimals); // Validate that the mint is indeed an NFT

        // Ensure payment currency is SOL
        require!(
            self.listing.payment_currency == PaymentCurrency::Sol,
            MarketplaceError::InvalidPaymentCurrency
        );
        
        // Ensure fan has sufficient funds
        require!(
            self.fan.lamports() >= self.listing.price,
            MarketplaceError::InsufficientFunds
        );
        
        let fee = self.listing.price.checked_mul(self.marketplace.fee as u64).unwrap() / 10_000;

        let payout = self.listing.price.checked_sub(fee).unwrap();

        // Transfer payout to artist
        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.fan.to_account_info(),
                    to: self.artist.to_account_info(),
                },
            ),
            payout,
        )?;
        
        // Transfer fee to treasury
        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.fan.to_account_info(),
                    to: self.treasury.to_account_info(),
                },
            ),
            fee,
        )?;
            
        Ok(())
    }

    fn pay_with_spl(&self, payout: u64, fee: u64) -> Result<()> {
        let mint = match self.listing.payment_currency {
            PaymentCurrency::Spl { mint } => mint,
            _ => return err!(MarketplaceError::InvalidPaymentCurrency),
        };
    
        require!(
            self.fan_ata.mint == mint,
            MarketplaceError::InvalidPaymentMint
        );

        // Ensure payment currency is SPL
        require!(
            matches!(self.listing.payment_currency, PaymentCurrency::Spl { .. }),
            MarketplaceError::InvalidPaymentCurrency
        );

        // Ensure fan has sufficient funds
        require!(
            self.fan_ata.amount >= self.listing.price,
            MarketplaceError::InsufficientFunds
        );
                    
        let token_program = self.token_program.to_account_info();
    
        // Pay artist
        transfer_checked(
            CpiContext::new(
                token_program.clone(),
                TransferChecked {
                    from: self.fan_ata.to_account_info(),
                    mint: self.mint_spl.to_account_info(),
                    to: self.artist_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            payout,
            self.rewards_mint.decimals,
        )?;
    
        // Pay treasury
        transfer_checked(
            CpiContext::new(
                token_program,
                TransferChecked {
                    from: self.fan_ata.to_account_info(),
                    mint: self.rewards_mint.to_account_info(),
                    to: self.treasury_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            fee,
            self.rewards_mint.decimals,
        )?;
    
        Ok(())
    }    
}
