use anchor_lang::prelude::*;
///use anchor_spl::token_interface::{Mint, TokenInterface};

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked},
    associated_token::AssociatedToken,
};

use crate::state::marketplace::Marketplace;
use crate::state::listing::Listing;

#[derive(Accounts)]
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
 
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>, // Reward token mint for the marketplace

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

