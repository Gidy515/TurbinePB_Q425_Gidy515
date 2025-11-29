#!allow[(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

declare_id!("A5xNuD9bvbdEy97ViGAq2pNQBxNhaMgZNYWkxMa2mhDp");

pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

#[program]
pub mod anchor_escrow {
    //use anchor_lang::prelude::borsh::de;

    use crate::instructions::Make;

    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, deposit: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>, _seed: u64) -> Result<()> {
        ctx.accounts.refund_and_close()?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>, _seed: u64) -> Result<()> {
        ctx.accounts.take()?;
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
