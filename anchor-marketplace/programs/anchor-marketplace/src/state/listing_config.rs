// this represents the seller_config that each seller will have per nft offer

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ListingConfig {
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub created_at: i64,
    pub expires_at: i64,
    pub is_active: bool,
    pub nft_vault: Pubkey,
    pub bump: u8,
}