use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PaymentCurrency {
    Sol,
    Spl { mint: Pubkey },
}

#[account]

pub struct Listing {
    pub artist: Pubkey,
    pub artist_mint: Pubkey,
    pub price: u64,
    pub bump: u8,
    pub marketplace: Pubkey,
    pub payment_currency: PaymentCurrency,
}

/*impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}*/

impl Space for Listing {
     const INIT_SPACE: usize =
            8 +   // discriminator
            32 +  // artist
            32 +  // artist_mint
            32 +  // marketplace
            8  +  // price
            1  +  // enum tag
            32 +  // SPL mint (worst case)
            1;    // bump
}


/*
1. Validate listing is active
2. Validate NFT is still in vault
3. Match payment currency
4. Transfer funds
5. Transfer NFT
6. Close vault
7. Close listing

 */