/*use anchor_lang::{
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
    pub fan_ata: InterfaceAccount<'info, TokenAccount>, 

    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>, // escrow holding NFT

    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), artist_mint.key().as_ref()],
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
    pub treasury_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    //#[account(mut)]
    //pub treasury_ata: Option<InterfaceAccount<'info, TokenAccount>>, // marketplace fee vault ATA


    /*#[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_mint_bump,
        mint::authority = marketplace,
        mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,*/

    /*#[account(
        mint::token_program = token_program
    )]
    pub mint_spl: InterfaceAccount<'info, Mint>,*/

    pub mint_spl: Option<InterfaceAccount<'info, Mint>>, // payment mint if SPL


    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


impl <'info> Purchase<'info>{
    pub fn purchase(&mut self) -> Result<()> {
        let fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fee as u64)
            .ok_or(MarketplaceError::MathOverflow)?
            / 10_000;
    
        let payout = self
            .listing
            .price
            .checked_sub(fee)
            .ok_or(MarketplaceError::MathOverflow)?;
    
        match self.listing.payment_currency {
            PaymentCurrency::Sol => {
                self.pay_in_sol()?;
            }
            PaymentCurrency::Spl { .. } => {
                self.pay_with_spl(payout, fee)?;
            }
        }
    
        self.send_nft_and_close_vault()?;
        Ok(())
    }

    pub fn pay_in_sol(&mut self) ->Result<()>{

        // Ensure treasury PDA is correct and belongs to this marketplace
        /*require!(
            self.treasury.key() == Pubkey::create_program_address(
                &[b"treasury", self.marketplace.key().as_ref(), &[self.marketplace.treasury_bump]],
                &crate::ID
            ).map_err(|_| MarketplaceError::InvalidTreasury)?,
            MarketplaceError::InvalidTreasury
        );*/

        /*let cpi_program = self.system_program.to_account_info();

        let cpi_accounts= Transfer{
            from: self.fan.to_account_info(), // From buyer
            to: self.artist.to_account_info(), // To seller
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);*/
        // Validate listing
        /*require!(self.listing.marketplace == self.marketplace.key(), MarketplaceError::InvalidMarketplaceListing); // Ensure the marketplace matches the listing
        require!(self.listing.artist == self.artist.key(), MarketplaceError::WrongArtist); // Ensure the artist is the original lister
        require!(self.listing.artist_mint == self.artist_mint.key(), MarketplaceError::WrongTokenMint); // Ensure the mint matches the listing

        // Validate that the vault holds the NFT
        require!(self.vault.amount == 1, MarketplaceError::WrongVaultAmount); // Ensure the vault holds exactly one token (the NFT) to prevent double spending
        require!(self.artist_mint.supply == 1, MarketplaceError::WrongMintSupply); // Validate that the mint is indeed an NFT
        require!(self.artist_mint.decimals == 0, MarketplaceError::WrongMintDecimals); // Validate that the mint is indeed an NFT*/
        // Listing integrity
        require!(self.listing.active, MarketplaceError::ListingInactive);
        require!(self.listing.marketplace == self.marketplace.key(), MarketplaceError::InvalidMarketplaceListing);
        require!(self.listing.artist == self.artist.key(), MarketplaceError::WrongArtist);
        require!(self.listing.artist_mint == self.artist_mint.key(), MarketplaceError::WrongTokenMint);

        // NFT validity
        require!(self.artist_mint.supply == 1, MarketplaceError::WrongMintSupply);
        require!(self.artist_mint.decimals == 0, MarketplaceError::WrongMintDecimals);
        require!(self.vault.amount == 1, MarketplaceError::WrongVaultAmount);

        // Payment currency
        require!(
            self.listing.payment_currency == PaymentCurrency::Sol,
            MarketplaceError::InvalidPaymentCurrency
        );

        // Balance check for sufficient funds
        require!(
            self.fan.lamports() >= self.listing.price,
            MarketplaceError::InsufficientFunds
        );
        
        let fee = self.listing.price.checked_mul(self.marketplace.fee as u64).ok_or(MarketplaceError::MathOverflow)? / 10_000;
    
        let payout = self.listing.price.checked_sub(fee).ok_or(MarketplaceError::MathOverflow)?;
        
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
        self.listing.active = false;

        Ok(())
    }

    /*fn pay_with_spl(&mut self, payout: u64, fee: u64) -> Result<()> {
        // Fan ATA must match payment mint
        require!(
            self.fan_ata.mint == self.mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );

        // Artist ATA must match payment mint
        require!(
            self.artist_ata.mint == self.mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );

        let mint = match self.listing.payment_currency {
            PaymentCurrency::Spl { mint } => mint,
            _ => return err!(MarketplaceError::InvalidPaymentCurrency),
        };

        require!(self.listing.active, MarketplaceError::ListingInactive);
    
        // Ensure correct payment mint
        require!(self.mint_spl.key() == mint, MarketplaceError::InvalidPaymentMint);
    
        // Ensure fan has funds
        require!(
            self.fan_ata.amount >= payout.checked_add(fee).ok_or(MarketplaceError::MathOverflow)?,
            MarketplaceError::InsufficientFunds
        );
    
        let token_program = self.token_program.to_account_info();
        let decimals = self.mint_spl.decimals;
    
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
            decimals,
        )?;
    
        // Pay treasury

        // Treasury ATA must match payment mint
        require!(
            self.treasury_ata.unwrap().mint == self.mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );

        // Treasury must be correct PDA
        require!(
            self.treasury_ata.unwrap().owner == self.treasury.key(),
            MarketplaceError::InvalidTreasury
        );        

        transfer_checked(
            CpiContext::new(
                token_program,
                TransferChecked {
                    from: self.fan_ata.to_account_info(),
                    mint: self.mint_spl.to_account_info(),
                    to: self.treasury_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            fee,
            decimals,
        )?;
        self.listing.active = false;

        Ok(())
    }*/
    /*fn pay_with_spl(&mut self, payout: u64, fee: u64) -> Result<()> {
        let mint_spl = self
            .mint_spl
            .as_ref()
            .ok_or(MarketplaceError::InvalidPaymentMint)?;

        let treasury_ata = self
            .treasury_ata
            .as_ref()
            .ok_or(MarketplaceError::InvalidTreasury)?;
    
        // Fan ATA must match payment mint
        require!(
            self.fan_ata.mint == self.mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
    
        // Artist ATA must match payment mint
        require!(
            self.artist_ata.mint == self.mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
    
        let mint = match self.listing.payment_currency {
            PaymentCurrency::Spl { mint } => mint,
            _ => return err!(MarketplaceError::InvalidPaymentCurrency),
        };
    
        require!(self.listing.active, MarketplaceError::ListingInactive);
        require!(self.mint_spl.key() == mint, MarketplaceError::InvalidPaymentMint);
    
        require!(
            self.fan_ata.amount >= payout.checked_add(fee).ok_or(MarketplaceError::MathOverflow)?,
            MarketplaceError::InsufficientFunds
        );
    
        let decimals = self.mint_spl.decimals;
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
            decimals,
        )?;
    
        // Validate treasury ATA
        require!(
            treasury_ata.mint == self.mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
    
        require!(
            treasury_ata.owner == self.treasury.key(),
            MarketplaceError::InvalidTreasury
        );
    
        // Pay treasury
        transfer_checked(
            CpiContext::new(
                token_program,
                TransferChecked {
                    from: self.fan_ata.to_account_info(),
                    mint: self.mint_spl.to_account_info(),
                    to: treasury_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            fee,
            decimals,
        )?;
    
        self.listing.active = false;
        Ok(())
    }*/
    fn pay_with_spl(&mut self, payout: u64, fee: u64) -> Result<()> {
        let mint_spl = self
            .mint_spl
            .as_ref()
            .ok_or(MarketplaceError::InvalidPaymentMint)?;
    
        let treasury_ata = self
            .treasury_ata
            .as_ref()
            .ok_or(MarketplaceError::InvalidTreasury)?;
    
        require!(self.listing.active, MarketplaceError::ListingInactive);
    
        let mint = match self.listing.payment_currency {
            PaymentCurrency::Spl { mint } => mint,
            _ => return err!(MarketplaceError::InvalidPaymentCurrency),
        };
    
        require!(mint_spl.key() == mint, MarketplaceError::InvalidPaymentMint);
    
        require!(
            self.fan_ata.amount >= payout.checked_add(fee).ok_or(MarketplaceError::MathOverflow)?,
            MarketplaceError::InsufficientFunds
        );
    
        let decimals = mint_spl.decimals;
        let token_program = self.token_program.to_account_info();
    
        // Pay artist
        transfer_checked(
            CpiContext::new(
                token_program.clone(),
                TransferChecked {
                    from: self.fan_ata.to_account_info(),
                    mint: mint_spl.to_account_info(),
                    to: self.artist_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            payout,
            decimals,
        )?;
    
        require!(treasury_ata.mint == mint_spl.key(), MarketplaceError::InvalidPaymentMint);
        require!(treasury_ata.owner == self.treasury.key(), MarketplaceError::InvalidTreasury);
    
        transfer_checked(
            CpiContext::new(
                token_program,
                TransferChecked {
                    from: self.fan_ata.to_account_info(),
                    mint: mint_spl.to_account_info(),
                    to: treasury_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            fee,
            decimals,
        )?;
    
        self.listing.active = false;
        Ok(())
    }
    
    
    
    pub fn send_nft_and_close_vault(&mut self) -> Result<()> {
        // Vault must be for the listed NFT mint
        require!(
            self.vault.mint == self.artist_mint.key(),
            MarketplaceError::WrongTokenMint
        );

        // Fan ATA must be for the NFT mint
        require!(
            self.fan_ata.mint == self.artist_mint.key(),
            MarketplaceError::WrongTokenMint
        );
    
        // Vault must still contain the NFT
        require!(self.vault.amount == 1, MarketplaceError::WrongVaultAmount);
    
        /*let seeds = &[
            &self.artist_mint.key().to_bytes()[..],
            &self.marketplace.key().to_bytes()[..],
            &[self.listing.bump],
        ];*/
        let seeds = &[
        &self.marketplace.key().to_bytes()[..],  // ✅ CORRECT
        &self.artist_mint.key().to_bytes()[..],   // ✅ CORRECT
        &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];
    
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.fan_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.artist_mint.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
    
        transfer_checked(cpi_ctx, 1, self.artist_mint.decimals)?;
    
        // Closing of the account
        
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.artist_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];
    
        let cpi_program = self.token_program.to_account_info();
    
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.artist.to_account_info(),
            authority: self.listing.to_account_info(),
        };
    
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds,
        );
    
        close_account(cpi_ctx)?;
    
        Ok(())
    }
        
}*/
/*use anchor_lang::{
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

    // NFT ATA for artist (used only for validation in SOL purchases)
    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = artist,
        associated_token::token_program = token_program,
    )]
    pub artist_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    // NFT ATA for fan (receives the NFT)
    #[account(
        init_if_needed,
        payer = fan,
        associated_token::mint = artist_mint,
        associated_token::authority = fan
    )]
    pub fan_nft_ata: InterfaceAccount<'info, TokenAccount>,

    // Vault holding the NFT during listing
    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), artist_mint.key().as_ref()],
        bump = listing.bump,
        close = artist
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,

    // ============ SPL-SPECIFIC ACCOUNTS ============
    
    // Payment mint (only used for SPL purchases)
    pub mint_spl: Option<InterfaceAccount<'info, Mint>>,

    // Fan's payment token ATA (only used for SPL purchases)
    #[account(mut)]
    pub fan_payment_ata: Option<InterfaceAccount<'info, TokenAccount>>,

    // Artist's payment token ATA (only used for SPL purchases)
    #[account(mut)]
    pub artist_payment_ata: Option<InterfaceAccount<'info, TokenAccount>>,

    // Treasury's payment token ATA (only used for SPL purchases)
    #[account(
        mut,
        associated_token::mint = mint_spl,
        associated_token::authority = treasury,
        associated_token::token_program = token_program,
    )]
    pub treasury_ata: Option<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn purchase(&mut self) -> Result<()> {
        let fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fee as u64)
            .ok_or(MarketplaceError::MathOverflow)?
            / 10_000;

        let payout = self
            .listing
            .price
            .checked_sub(fee)
            .ok_or(MarketplaceError::MathOverflow)?;

        match self.listing.payment_currency {
            PaymentCurrency::Sol => {
                self.pay_in_sol()?;
            }
            PaymentCurrency::Spl { .. } => {
                self.pay_with_spl(payout, fee)?;
            }
        }

        self.send_nft_and_close_vault()?;
        Ok(())
    }

    pub fn pay_in_sol(&mut self) -> Result<()> {
        // Listing integrity
        require!(self.listing.active, MarketplaceError::ListingInactive);
        require!(
            self.listing.marketplace == self.marketplace.key(),
            MarketplaceError::InvalidMarketplaceListing
        );
        require!(
            self.listing.artist == self.artist.key(),
            MarketplaceError::WrongArtist
        );
        require!(
            self.listing.artist_mint == self.artist_mint.key(),
            MarketplaceError::WrongTokenMint
        );

        // NFT validity
        require!(
            self.artist_mint.supply == 1,
            MarketplaceError::WrongMintSupply
        );
        require!(
            self.artist_mint.decimals == 0,
            MarketplaceError::WrongMintDecimals
        );
        require!(self.vault.amount == 1, MarketplaceError::WrongVaultAmount);

        // Payment currency
        require!(
            self.listing.payment_currency == PaymentCurrency::Sol,
            MarketplaceError::InvalidPaymentCurrency
        );

        // Balance check for sufficient funds
        require!(
            self.fan.lamports() >= self.listing.price,
            MarketplaceError::InsufficientFunds
        );

        let fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fee as u64)
            .ok_or(MarketplaceError::MathOverflow)?
            / 10_000;

        let payout = self
            .listing
            .price
            .checked_sub(fee)
            .ok_or(MarketplaceError::MathOverflow)?;

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

        self.listing.active = false;
        Ok(())
    }

    fn pay_with_spl(&mut self, payout: u64, fee: u64) -> Result<()> {
        let mint_spl = self
            .mint_spl
            .as_ref()
            .ok_or(MarketplaceError::InvalidPaymentMint)?;

        let fan_payment_ata = self
            .fan_payment_ata
            .as_ref()
            .ok_or(MarketplaceError::InvalidPaymentMint)?;

        let artist_payment_ata = self
            .artist_payment_ata
            .as_ref()
            .ok_or(MarketplaceError::InvalidPaymentMint)?;

        let treasury_ata = self
            .treasury_ata
            .as_ref()
            .ok_or(MarketplaceError::InvalidTreasury)?;

        require!(self.listing.active, MarketplaceError::ListingInactive);

        let mint = match self.listing.payment_currency {
            PaymentCurrency::Spl { mint } => mint,
            _ => return err!(MarketplaceError::InvalidPaymentCurrency),
        };

        require!(
            mint_spl.key() == mint,
            MarketplaceError::InvalidPaymentMint
        );

        // Validate fan's payment ATA
        require!(
            fan_payment_ata.mint == mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
        require!(
            fan_payment_ata.owner == self.fan.key(),
            MarketplaceError::InvalidPaymentMint
        );

        // Validate artist's payment ATA
        require!(
            artist_payment_ata.mint == mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
        require!(
            artist_payment_ata.owner == self.artist.key(),
            MarketplaceError::InvalidPaymentMint
        );

        require!(
            fan_payment_ata.amount
                >= payout
                    .checked_add(fee)
                    .ok_or(MarketplaceError::MathOverflow)?,
            MarketplaceError::InsufficientFunds
        );

        let decimals = mint_spl.decimals;
        let token_program = self.token_program.to_account_info();

        // Pay artist
        transfer_checked(
            CpiContext::new(
                token_program.clone(),
                TransferChecked {
                    from: fan_payment_ata.to_account_info(),
                    mint: mint_spl.to_account_info(),
                    to: artist_payment_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            payout,
            decimals,
        )?;

        // Validate treasury ATA
        require!(
            treasury_ata.mint == mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
        require!(
            treasury_ata.owner == self.treasury.key(),
            MarketplaceError::InvalidTreasury
        );

        // Pay treasury
        transfer_checked(
            CpiContext::new(
                token_program,
                TransferChecked {
                    from: fan_payment_ata.to_account_info(),
                    mint: mint_spl.to_account_info(),
                    to: treasury_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            fee,
            decimals,
        )?;

        self.listing.active = false;
        Ok(())
    }

    pub fn send_nft_and_close_vault(&mut self) -> Result<()> {
        // Vault must be for the listed NFT mint
        require!(
            self.vault.mint == self.artist_mint.key(),
            MarketplaceError::WrongTokenMint
        );

        // Fan NFT ATA must be for the NFT mint
        require!(
            self.fan_nft_ata.mint == self.artist_mint.key(),
            MarketplaceError::WrongTokenMint
        );

        // Vault must still contain the NFT
        require!(self.vault.amount == 1, MarketplaceError::WrongVaultAmount);

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.artist_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.fan_nft_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.artist_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(cpi_ctx, 1, self.artist_mint.decimals)?;

        // Close the vault
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.artist_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.artist.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}*/
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
    pub fan: Signer<'info>,

    #[account(mut)]
    pub artist: SystemAccount<'info>,

    pub artist_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = artist,
        associated_token::token_program = token_program,
    )]
    pub artist_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>, // ✅ Box this

    #[account(
        init_if_needed,
        payer = fan,
        associated_token::mint = artist_mint,
        associated_token::authority = fan
    )]
    pub fan_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = artist_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), artist_mint.key().as_ref()],
        bump = listing.bump,
        close = artist
    )]
    pub listing: Box<Account<'info, Listing>>, // ✅ Box this

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,

    // ============ SPL-SPECIFIC ACCOUNTS ============
    
    pub mint_spl: Option<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub fan_payment_ata: Option<Box<InterfaceAccount<'info, TokenAccount>>>, // ✅ Box this

    #[account(mut)]
    pub artist_payment_ata: Option<Box<InterfaceAccount<'info, TokenAccount>>>, // ✅ Box this

    #[account(
        mut,
        associated_token::mint = mint_spl,
        associated_token::authority = treasury,
        associated_token::token_program = token_program,
    )]
    pub treasury_ata: Option<Box<InterfaceAccount<'info, TokenAccount>>>, // ✅ Box this

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn purchase(&mut self) -> Result<()> {
        let fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fee as u64)
            .ok_or(MarketplaceError::MathOverflow)?
            / 10_000;

        let payout = self
            .listing
            .price
            .checked_sub(fee)
            .ok_or(MarketplaceError::MathOverflow)?;

        match self.listing.payment_currency {
            PaymentCurrency::Sol => {
                self.pay_in_sol()?;
            }
            PaymentCurrency::Spl { .. } => {
                self.pay_with_spl(payout, fee)?;
            }
        }

        self.send_nft_and_close_vault()?;
        Ok(())
    }

    pub fn pay_in_sol(&mut self) -> Result<()> {
        require!(self.listing.active, MarketplaceError::ListingInactive);
        require!(
            self.listing.marketplace == self.marketplace.key(),
            MarketplaceError::InvalidMarketplaceListing
        );
        require!(
            self.listing.artist == self.artist.key(),
            MarketplaceError::WrongArtist
        );
        require!(
            self.listing.artist_mint == self.artist_mint.key(),
            MarketplaceError::WrongTokenMint
        );

        require!(
            self.artist_mint.supply == 1,
            MarketplaceError::WrongMintSupply
        );
        require!(
            self.artist_mint.decimals == 0,
            MarketplaceError::WrongMintDecimals
        );
        require!(self.vault.amount == 1, MarketplaceError::WrongVaultAmount);

        require!(
            self.listing.payment_currency == PaymentCurrency::Sol,
            MarketplaceError::InvalidPaymentCurrency
        );

        require!(
            self.fan.lamports() >= self.listing.price,
            MarketplaceError::InsufficientFunds
        );

        let fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fee as u64)
            .ok_or(MarketplaceError::MathOverflow)?
            / 10_000;

        let payout = self
            .listing
            .price
            .checked_sub(fee)
            .ok_or(MarketplaceError::MathOverflow)?;

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

        self.listing.active = false;
        Ok(())
    }

    fn pay_with_spl(&mut self, payout: u64, fee: u64) -> Result<()> {
        let mint_spl = self
            .mint_spl
            .as_ref()
            .ok_or(MarketplaceError::InvalidPaymentMint)?;

        let fan_payment_ata = self
            .fan_payment_ata
            .as_ref()
            .ok_or(MarketplaceError::InvalidPaymentMint)?;

        let artist_payment_ata = self
            .artist_payment_ata
            .as_ref()
            .ok_or(MarketplaceError::InvalidPaymentMint)?;

        let treasury_ata = self
            .treasury_ata
            .as_ref()
            .ok_or(MarketplaceError::InvalidTreasury)?;

        require!(self.listing.active, MarketplaceError::ListingInactive);

        let mint = match self.listing.payment_currency {
            PaymentCurrency::Spl { mint } => mint,
            _ => return err!(MarketplaceError::InvalidPaymentCurrency),
        };

        require!(
            mint_spl.key() == mint,
            MarketplaceError::InvalidPaymentMint
        );

        require!(
            fan_payment_ata.mint == mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
        require!(
            fan_payment_ata.owner == self.fan.key(),
            MarketplaceError::InvalidPaymentMint
        );

        require!(
            artist_payment_ata.mint == mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
        require!(
            artist_payment_ata.owner == self.artist.key(),
            MarketplaceError::InvalidPaymentMint
        );

        require!(
            fan_payment_ata.amount
                >= payout
                    .checked_add(fee)
                    .ok_or(MarketplaceError::MathOverflow)?,
            MarketplaceError::InsufficientFunds
        );

        let decimals = mint_spl.decimals;
        let token_program = self.token_program.to_account_info();

        transfer_checked(
            CpiContext::new(
                token_program.clone(),
                TransferChecked {
                    from: fan_payment_ata.to_account_info(),
                    mint: mint_spl.to_account_info(),
                    to: artist_payment_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            payout,
            decimals,
        )?;

        require!(
            treasury_ata.mint == mint_spl.key(),
            MarketplaceError::InvalidPaymentMint
        );
        require!(
            treasury_ata.owner == self.treasury.key(),
            MarketplaceError::InvalidTreasury
        );

        transfer_checked(
            CpiContext::new(
                token_program,
                TransferChecked {
                    from: fan_payment_ata.to_account_info(),
                    mint: mint_spl.to_account_info(),
                    to: treasury_ata.to_account_info(),
                    authority: self.fan.to_account_info(),
                },
            ),
            fee,
            decimals,
        )?;

        self.listing.active = false;
        Ok(())
    }

    pub fn send_nft_and_close_vault(&mut self) -> Result<()> {
        require!(
            self.vault.mint == self.artist_mint.key(),
            MarketplaceError::WrongTokenMint
        );

        require!(
            self.fan_nft_ata.mint == self.artist_mint.key(),
            MarketplaceError::WrongTokenMint
        );

        require!(self.vault.amount == 1, MarketplaceError::WrongVaultAmount);

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.artist_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.fan_nft_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.artist_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(cpi_ctx, 1, self.artist_mint.decimals)?;

        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.artist_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.artist.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}
