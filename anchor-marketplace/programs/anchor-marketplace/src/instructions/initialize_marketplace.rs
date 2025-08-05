use anchor_lang::prelude::*;

use crate::MarketplaceConfig;

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + MarketplaceConfig::INIT_SPACE,
        seeds = [b"marketplace", authority.key().as_ref()],
        bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMarketplace<'info> {
    pub fn initialize_marketplace(&mut self, platform_fee: u8) -> Result<()> {
        self.marketplace_config.set_inner(
            MarketplaceConfig { 
            authority: *self.authority.key, 
            platform_fee: platform_fee, 
            bump: self.marketplace_config.bump
        }
    );
    Ok(())
}
}
