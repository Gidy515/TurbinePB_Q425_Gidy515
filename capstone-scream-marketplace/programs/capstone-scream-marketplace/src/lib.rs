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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
