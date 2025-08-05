use anchor_lang::prelude::*;

// global config for marketplace
// this is a singular marketplace which will be initialized only once
// sellers can post their offers with asked_price
// buyers can buy
// sellers can set the time for which the listing will be active

#[derive(InitSpace)]
#[account]
pub struct MarketplaceConfig {
    pub authority: Pubkey,          // marketplace initializer who will have the authority to make some changes later
    pub platform_fee: u8,
    pub bump: u8,
}