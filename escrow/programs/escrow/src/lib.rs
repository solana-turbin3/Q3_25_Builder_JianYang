#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;



declare_id!("3kuaUHbR3PRgGv3BrUnmvS4gfPeVFQYwuiKciQBUTA1J");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, give: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, give, receive, &ctx.bumps)?;
        ctx.accounts.deposit(give)
    }

    pub fn take(ctx: Context<Take>,) -> Result<()> {
        ctx.accounts.taker_to_maker()?;
        ctx.accounts.release_and_close_vault()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

}
