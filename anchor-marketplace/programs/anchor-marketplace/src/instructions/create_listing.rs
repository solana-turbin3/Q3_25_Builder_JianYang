use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount}};

use crate::{ListingConfig};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller,
    )]
    pub seller_nft_account: Account<'info, TokenAccount>,
    // listing ac that will take care of listing of a particular offer
    #[account(
        init,
        payer = seller,
        space = 8 + ListingConfig::INIT_SPACE,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, ListingConfig>,
    #[account(
        init,
        payer = seller,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub nft_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl <'info> CreateListing<'info> {
    pub fn create_listing(
        &mut self, 
        price: u64,
        listing_period_in_hours: i64,
    ) -> Result<()> {
        let current_timestamp = Clock::get()?.unix_timestamp;
        let listing_period_in_seconds = listing_period_in_hours
            .checked_mul(3600_i64)
            .ok_or(ErrorCode::ArithmeticOverflowInCreateListing)?;
        let listing_expiry_timestamp = listing_period_in_seconds
            .checked_add(current_timestamp)
            .ok_or(ErrorCode::ArithmeticOverflowInCreateListing)?;

        // NFT transfer from seller to the nft_vault
        let cpi_accounts = token::Transfer {
            authority: self.seller.to_account_info(),
            from: self.seller_nft_account.to_account_info(),
            to: self.nft_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;
        
        self.listing.set_inner(
            ListingConfig { 
                nft_mint: self.nft_mint.key(), 
                seller: *self.seller.key, 
                price: price, 
                created_at: current_timestamp,
                expires_at: listing_expiry_timestamp, 
                is_active: true, 
                nft_vault: self.nft_vault.key(), 
                bump: self.listing.bump
             }
        );
        Ok(())
    }
}