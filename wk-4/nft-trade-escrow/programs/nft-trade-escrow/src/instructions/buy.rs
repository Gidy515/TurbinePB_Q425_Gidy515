use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken}, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account}
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub seller: SystemAccount<'info>,
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
        associated_token::mint = mint_spl,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_spl_ata: InterfaceAccount<'info, TokenAccount>,
    

    #[account(
        mut,
        associated_token::mint = mint_nft,
        associated_token::authority = buyer,
        associated_token::token_program = token_program,
    )]
    pub buyer_nft_ata: InterfaceAccount<'info, TokenAccount>,


    #[account(
        mut,
        associated_token::mint = mint_spl,
        associated_token::authority = seller,
        associated_token::token_program = token_program,
    )]
    pub seller_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = seller,
        has_one = mint_nft,
        has_one = mint_spl,
        has_one = seller,
        seeds = [b"escrow", seller.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_spl,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info> Buy<'info> {
    pub fn take(&mut self) -> Result<()> {

        let transfer_accounts = TransferChecked {
            from: self.buyer_spl_ata.to_account_info(),
            mint: self.mint_spl.to_account_info(),
            to: self.seller_nft_ata.to_account_info(),
            authority: self.buyer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive_amount, self.mint_spl.decimals);

        Ok(())
    }

    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        //let maker_key = self.maker.key();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.seller.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.buyer_spl_ata.to_account_info(),
            authority: self.escrow.to_account_info(), // authority should be maker
            mint: self.mint_nft.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            &signer_seeds,
        );

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_nft.decimals);

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.seller.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            &signer_seeds,
        );

        close_account(close_cpi_ctx);

        Ok(())
    }
}