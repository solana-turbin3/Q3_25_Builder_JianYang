use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic Overflow In CreateListing")]
    ArithmeticOverflowInCreateListing,
    #[msg("Unauthorized Seller Trying To Remove Listing")]
    UnauthorizedSeller,
    #[msg("Invalid Seller")]
    InvalidSeller,
    #[msg("Listing Expired")]
    ListingExpired,
    #[msg("Invalid Authority")]
    InvalidAuthority,
    #[msg("Listing Not Active")]
    ListingNotActive,
}
