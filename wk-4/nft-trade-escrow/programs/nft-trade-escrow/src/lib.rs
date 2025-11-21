#!allow[(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

declare_id!("7HgZjih81tySt3y7LxXehfSYGM3yQsAZwZ2fbvi5j2Ci");

pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;
use state::*;

#[program]
pub mod nft_trade_escrow {
    use super::*;
    
    pub fn sell(
        mut ctx: Context<Sell>,
        seed: u64,
        sell_amount: u64,
        receive_amount: u64,
    ) -> Result<()> {
        let accounts = &mut ctx.accounts;

        accounts.init_escrow(seed, sell_amount, receive_amount, &ctx.bumps)?;
        accounts.deposit(sell_amount)?;

        Ok(())
    }

    pub fn buy(
        mut ctx: Context<Buy>,
        seed: u64,
    ) -> Result<()> {
        let accounts = &mut ctx.accounts;

        accounts.buy()?;
        accounts.withdraw_and_close_vault()?;

        Ok(())
    }

    pub fn refund(
        mut ctx: Context<Refund>,
        seed: u64,
    ) -> Result<()> {
        let accounts = &mut ctx.accounts;

        accounts.refund_and_close()?;

        Ok(())
    }
}
