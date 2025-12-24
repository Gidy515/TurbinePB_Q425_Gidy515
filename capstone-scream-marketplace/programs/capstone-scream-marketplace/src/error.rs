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

    #[msg("Free listings are not currently allowed in this marketplace")]
    InvalidPrice,

    #[msg("Please initialize the marketplace before listing NFTs")]
    InvalidMarketplaceState,

    #[msg("This NFT is already listed")]
    AlreadyListed,

    #[msg("You do not own this NFT")]
    InvalidNftOwnership,

    #[msg("Token amount must be exactly 1 for NFTs")]
    InvalidMintDecimals, InvalidMintSupply, InvalidTokenAmount,

    #[msg("The vault must be empty before depositing an NFT")]
    VaultNotEmpty,

    #[msg("Must own the NFT to deposit it into the vault")]
    InvalidTokenOwner,

    #[msg("Error during token transfer for listing NFT")]
    NftTransferFailed,
}