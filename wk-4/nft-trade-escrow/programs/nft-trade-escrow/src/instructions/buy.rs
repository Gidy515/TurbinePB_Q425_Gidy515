use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken}, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account}
};

use crate::state::Escrow;

use crate::error::ErrorCode;

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

impl<'info> Buy<'info> {
    pub fn buy(&mut self) -> Result<()> {
        // 1. Validate buyer balance
        require!(
            self.buyer_spl_ata.amount >= self.escrow.receive_amount,
            ErrorCode::InsufficientBuyerBalance
        );

        // 2. Transfer SPL tokens from buyer -> seller
        let pay_seller_accounts = TransferChecked {
            from: self.buyer_spl_ata.to_account_info(),
            mint: self.mint_spl.to_account_info(),
            to: self.seller_nft_ata.to_account_info(),
            authority: self.buyer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), pay_seller_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive_amount, self.mint_spl.decimals)?;

        self.escrow.is_fulfilled = true;

        Ok(())
    }

    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        // PDA signer
        let signer_seeds: &[&[u8]] = &[
            b"escrow",
            self.seller.key.as_ref(),
            &self.escrow.seed.to_le_bytes(),
            &[self.escrow.bump],
        ];

        let signer = &[signer_seeds];

        // 3. Transfer NFT from vault PDA -> buyer
        let withdraw_nft_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_nft.to_account_info(),
            to: self.buyer_nft_ata.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            withdraw_nft_accounts,
            signer,
        );

        transfer_checked(cpi_ctx, self.escrow.sell_amount, self.mint_nft.decimals)
            .map_err(|_| ErrorCode::FailedVaultWithdrawal)?;

        // 4. Close vault ATA
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.seller.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer,
        );

        close_account(close_ctx).map_err(|_| ErrorCode::FailedVaultClosure)?;

        Ok(())
    }
}
