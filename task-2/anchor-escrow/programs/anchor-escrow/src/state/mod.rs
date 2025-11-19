use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow { // this escrow does not hold the taker's pubkey because it's not needed until the taker takes the escrow
    pub seed: u64, // This is the seed used to derive the PDA: the seed is provided by the maker when creating the escrow
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}

