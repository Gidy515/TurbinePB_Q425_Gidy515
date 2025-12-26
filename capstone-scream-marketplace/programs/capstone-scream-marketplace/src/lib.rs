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
}


















