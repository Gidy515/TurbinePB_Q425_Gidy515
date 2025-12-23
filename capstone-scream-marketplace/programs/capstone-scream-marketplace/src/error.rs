use anchor_lang::error_code;
//use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Marketplace name is too short")]
    NameTooShort,

    #[msg("Name is too long")]
    NameTooLong,

    #[msg("Fee percentage is invalid")]
    InvalidFee,

    #[msg("Invalid marketplace name")]
    InvalidMarketplaceName,

    #[msg("Invalid Treasury address")]
    InvalidTreasury,

    #[msg("Invalid Rewards Mint address")]
    InvalidRewardsMint,

    #[msg("Invalid Rewards Mint Authority")]
    InvalidRewardsMintAuthority,

}