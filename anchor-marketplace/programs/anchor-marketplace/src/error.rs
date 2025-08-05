use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic Overflow In CreateListing")]
    ArithmeticOverflowInCreateListing,
    #[msg("Unauthorized Seller Trying To Remove Listing")]
    UnauthorizedSeller,
}
