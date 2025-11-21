use anchor_lang::prelude::*;

use anchor_spl::{
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked},
    associated_token::AssociatedToken,
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Sell<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_nft: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_spl: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_nft,
        associated_token::authority = seller,
        associated_token::token_program = token_program,
    )]
    pub seller_nft_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_nft,
        associated_token::authority = seller,
        associated_token::token_program = token_program
    )]
      pub seller_spl_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = seller,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", seller.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow_offer: Account<'info, Escrow>,
    #[account(
        init,
        payer = seller,
        associated_token::mint = mint_nft,
        associated_token::authority = escrow_offer,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info> Sell<'info> {
    pub fn init_escrow(&mut self, seed: u64, sell_amount: u64, receive_amount: u64, bumps: &SellBumps) -> Result<()> {
        self.escrow_offer.set_inner(Escrow {
            seed, 
            seller: self.seller.key(), 
            mint_nft: self.mint_nft.key(), 
            mint_spl: self.mint_spl.key(), 
            sell_amount,
            receive_amount, 
            bump: bumps.escrow_offer,
        });

        Ok(())
    }

    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.seller_nft_ata.to_account_info(),
            mint: self.mint_nft.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.seller.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, deposit, self.mint_nft.decimals)?;

        Ok(())
    }
}