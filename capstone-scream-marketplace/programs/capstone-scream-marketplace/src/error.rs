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

    #[msg("You are not authorized to delist this NFT")]
    UnauthorizedDelist,

    #[msg("The listing does not belong to this marketplace")]
    InvalidMarketplace,

    #[msg("This mint does not match the listing's artist mint")]
    InvalidArtistMint,

    #[msg("Invalid amount in the vault for withdrawal")]
    InvalidVaultAmount,

    #[msg("An error occurred closing the mint vault")]
    VaultCloseFailed,

    #[msg("The marketplace does not match the listing")]
    InvalidMarketplaceListing,

    #[msg("The specified artist does not match the listing")]
    WrongArtist,

    #[msg("The specified token mint does not match the listing")]
    WrongTokenMint,

    #[msg("The vault does not hold the correct amount of the NFT")]
    WrongVaultAmount,

    #[msg("The specified mint is not a valid NFT")]
    WrongMintSupply,

    #[msg("The specified mint does not have the correct decimals for an NFT")]
    WrongMintDecimals,

    #[msg("Invalid payment currency for this listing")]
    InvalidPaymentCurrency,

    #[msg("The payment mint does not match the listing's payment currency")]
    InvalidPaymentMint,

    #[msg("Insufficient funds to complete the purchase")]
    InsufficientFunds,

    #[msg("Mathematical operation overflowed")]
    MathOverflow,

    #[msg("NFT listing is not active")]
    ListingInactive,
}