use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount}};

use crate::{ListingConfig, MarketplaceConfig};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        constraint = seller.key() == listing.seller.key() @ ErrorCode::InvalidSeller
    )]
    pub seller: SystemAccount<'info>,
    #[account(
        mut,
        constraint = authority.key() == marketplace_config.authority.key() @ ErrorCode::InvalidAuthority
    )]
    pub authority: SystemAccount<'info>,        // the authority who created the marketplace
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing,
    )]
    pub nft_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump,
        close = seller,
        constraint = listing.is_active @ ErrorCode::ListingNotActive
    )]
    pub listing: Account<'info, ListingConfig>,
    #[account(
        seeds = [b"marketplace", authority.key().as_ref()],
        bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,
    #[account(
        init_if_needed,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer,
        payer = buyer,
    )]
    pub buyer_nft_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl <'info> Purchase<'info> {
    pub fn transfer_nft_to_buyer(&mut self) -> Result<()> {
        let cpi_accounts = token::Transfer {
            authority: self.listing.to_account_info(),
            from: self.nft_vault.to_account_info(),
            to: self.buyer_nft_account.to_account_info(),
        };
        let nft_mint_address = self.nft_mint.key();
        let seeds = &[b"listing", nft_mint_address.as_ref(), &[self.listing.bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, 1)?;
        Ok(())
    }

    pub fn fee_transfer(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time < self.listing.expires_at, ErrorCode::ListingExpired);
        let platform_fee = (self.listing.price * self.marketplace_config.platform_fee as u64) / 100;
        // sending the whole amount to the seller
        let transfer_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), transfer_accounts);
        anchor_lang::system_program::transfer(cpi_ctx, platform_fee)?;

        Ok(())
    }
    
    pub fn seller_amount_transfer(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time < self.listing.expires_at, ErrorCode::ListingExpired);
        let platform_fee = (self.listing.price * self.marketplace_config.platform_fee as u64) / 100;
        let seller_amount = self.listing.price - platform_fee;
        // sending the whole amount to the seller
        let transfer_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), transfer_accounts);
        anchor_lang::system_program::transfer(cpi_ctx, seller_amount)?;

        Ok(())
    }
}