use anchor_lang::prelude::*;

declare_id!("2d8HV2bbi5j59yxnnfsPphvQhGQHHbzyPasbDBLbkAmo");

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
