#![allow(deprecated)]
#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FqJ7dcjXWeo5hUfEWXcJLzrYoyUBy93CxWfeXZDYkjXZ");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, platform_fee: u8) -> Result<()> {
        ctx.accounts.initialize_marketplace(platform_fee)?;
        Ok(())
    }

    pub fn create_listing(ctx: Context<CreateListing>, price: u64, listing_period_in_hours: i64,) -> Result<()> {
        ctx.accounts.create_listing(price, listing_period_in_hours)?;
        Ok(())
    }

    pub fn remove_listing(ctx: Context<RemoveListing>) -> Result<()>{
        ctx.accounts.remove_listing()?;
        Ok(())
    }
}
