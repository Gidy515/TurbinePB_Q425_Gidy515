use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow { 
    pub seed: u64, 
    pub seller: Pubkey,
    pub mint_nft: Pubkey,
    pub mint_spl: Pubkey,
    pub sell_amount: u64,
    pub receive_amount: u64,
    pub bump: u8,
}