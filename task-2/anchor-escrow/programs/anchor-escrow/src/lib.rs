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

    pub fn take(ctx: Context<Make>, seed: u64, deposit: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
