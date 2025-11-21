use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient or no NFTs in balance seller's account")]
    InsufficientSellerBalance,

    #[msg("Insufficient tokens in balance buyer's account")]
    InsufficientBuyerBalance,

    #[msg("Invalid token mint - must be different from offered token")]
    InvalidTokenMint,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Failed to withdraw tokens from vault")]
    FailedVaultWithdrawal,

    #[msg("Cannot cancel escrow: buyer has already paid.")]
    CannotCancelAlreadyPurchased,

    #[msg("Failed to close vault account")]
    FailedVaultClosure,

    #[msg("Failed to refund tokens from vault")]
    FailedRefundTransfer,

    #[msg("Failed to close vault during refund")]
    FailedRefundClosure,
}