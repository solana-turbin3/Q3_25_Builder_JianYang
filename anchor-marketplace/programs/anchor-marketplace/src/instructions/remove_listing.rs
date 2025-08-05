use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount}};

use crate::ListingConfig;
use crate::error::ErrorCode;
#[derive(Accounts)]
pub struct RemoveListing<'info> {
    pub seller: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump,
        close = seller,
    )]
    pub listing: Account<'info, ListingConfig>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = seller,
    )]
    pub seller_nft_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub nft_vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl <'info> RemoveListing<'info> {
    pub fn remove_listing(&mut self) -> Result<()> {
        require!(
            self.seller.key() == self.listing.seller.key(),
            ErrorCode::UnauthorizedSeller,
        );
        self.listing.is_active = false;
        let nft_mint_key = self.nft_mint.key();
        let seeds = &[b"listing", nft_mint_key.as_ref(), &[self.listing.bump]];
        let cpi_accounts = token::Transfer {
            authority: self.listing.to_account_info(),
            from: self.nft_vault.to_account_info(),
            to: self.seller_nft_account.to_account_info(),
        };
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            cpi_accounts,
            signer_seeds
        );
        token::transfer(cpi_ctx, 1)?;
        
        Ok(())
    }
}