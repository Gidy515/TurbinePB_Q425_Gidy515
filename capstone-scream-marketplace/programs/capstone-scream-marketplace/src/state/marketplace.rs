use anchor_lang::prelude::*;

#[account]

pub struct Marketplace {
    pub name: String,
    pub admin: Pubkey,
    pub fee: u16, // fee in basis points (1/100 of a percent)
    pub bump: u8, // the marketplace bump
    pub treasury_bump: u8,
    pub rewards_mint_bump: u8,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 
                            8 // anchor discriminator 
                            + 32 // admin public key
                            + 2 // fee
                            + 1 // bump
                            + 1 // bump
                            + 1 // bump
                            + (4 + 32); // marketplace name (32 bytes for string length and 4 bytes for name content)
}