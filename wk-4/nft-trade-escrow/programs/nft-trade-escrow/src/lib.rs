use anchor_lang::prelude::*;

declare_id!("7HgZjih81tySt3y7LxXehfSYGM3yQsAZwZ2fbvi5j2Ci");

#[program]
pub mod nft_trade_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
