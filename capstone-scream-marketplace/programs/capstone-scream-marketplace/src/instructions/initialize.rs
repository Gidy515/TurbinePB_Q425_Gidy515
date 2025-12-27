use anchor_lang::prelude::*;
///use anchor_spl::token_interface::{Mint, TokenInterface};

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface},
    associated_token::AssociatedToken,
};

use crate::state::marketplace::Marketplace;
// use crate::state::listing::Listing;

// reaching the error module, errors.rs
use crate::error::MarketplaceError;

//const MIN_NAME_LEN: usize = 3;
//const MAX_NAME_LEN: usize = 32;
//const MAX_FEE_BPS: u16 = 1_000; // 10%


#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
        space = Marketplace::INIT_SPACE,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>, // account to hold marketplace fees

    /// SPL treasury ATA (only used if SPL)
    #[account(mut)]
    pub treasury_payment_ata: Option<InterfaceAccount<'info, TokenAccount>>,

 
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>, // Reward token mint for the marketplace

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info> Initialize<'info> {
    pub fn initialize (&mut self, name: String, fee: u16, bumps: InitializeBumps) -> Result<()> {
        // Validate name length
        let name_len = name.as_bytes().len();

        require!(
            name_len >= 3,
            MarketplaceError::NameTooShort
        );

        require!(
            name_len <= 32,
            MarketplaceError::NameTooLong
        );

        // fee sanity check 
        // Example: max 10%
        require!(
            fee <= 1_000,
            MarketplaceError::InvalidFee
        );

        // Validate name characters, so only lowercase letters, numbers, hyphens, and underscores are allowed so that it can be used in URLs
        require!(
            name.chars().all(|c| {
                c.is_ascii_lowercase()
                    || c.is_ascii_digit()
                    || c == '-'
                    || c == '_'
            }),
            MarketplaceError::InvalidMarketplaceName
        );

        // Treasury PDA correctness check
        let (expected_treasury, _) = Pubkey::find_program_address(
            &[b"treasury", self.marketplace.key().as_ref()],
            &crate::ID,
        );

        require!(
            self.treasury.key() == expected_treasury,
            MarketplaceError::ConstraintSeeds // Using generic error for PDA mismatch. ConstraintSeeds is a built-in Anchor error.
        );

        // Rewards mint PDA correctness check to ensure it matches expected PDA
        let (expected_rewards_mint, _) = Pubkey::find_program_address(
            &[b"rewards", self.marketplace.key().as_ref()],
            &crate::ID,
        );

        require!(
            self.rewards_mint.key() == expected_rewards_mint,
            MarketplaceError::InvalidRewardsMint
        );

        // Rewards mint authority check to ensure marketplace is the mint authority
        require!(
            self.rewards_mint
                .mint_authority
                .ok_or(MarketplaceError::InvalidRewardsMintAuthority)?
                == self.marketplace.key(),
            MarketplaceError::InvalidRewardsMintAuthority
        );

        // Persist marketplace data

        self.marketplace.set_inner(Marketplace {
            name,
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_mint_bump: bumps.rewards_mint,
        });
        
        Ok(())
    }
}