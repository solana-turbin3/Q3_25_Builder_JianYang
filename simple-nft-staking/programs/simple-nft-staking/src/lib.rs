// simple nft-staking program only demonstrating the custody of NFT
#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod state;
use anchor_spl::{token::{Mint, Token, TokenAccount},*};
pub use state::*;
declare_id!("8MHKEF5PAsHasEcPGcJqKRtWYp6h6QnJJXhA864jBRrF");

#[program]
pub mod simple_nft_staking {

    use super::*;

    pub fn stake(ctx: Context<Stake>, nft_mint: Pubkey) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        stake_account.staker = ctx.accounts.payer.key();
        stake_account.staked_at = Clock::get()?.unix_timestamp;
        stake_account.nft_mint = nft_mint;

        let cpi_accounts = token::Transfer {
            from: ctx.accounts.user_nft_account.to_account_info(),
            to: ctx.accounts.nft_vault.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, 1)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        require!(stake_account.staker == ctx.accounts.payer.key(), ErrorCode::UnauthorizedUnstake);
        require!(stake_account.staked_at > 0, ErrorCode::NotStaked);
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.nft_vault.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let program_id_bytes = id().to_bytes();
        let vault_authority_seeds: &[&[u8]] = &[
        b"vault_authority",
        &program_id_bytes,
        &[ctx.bumps.vault_authority],
        ];

    let signer_seeds = &[vault_authority_seeds];
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, 1)?;
        // updates data after cpi to reset the stats
        stake_account.staked_at = 0;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(nft_mint: Pubkey)]
pub struct Stake<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + StakeAccount::INIT_SPACE,
        seeds = [b"stake_account", nft_mint.key().as_ref(), payer.key().as_ref()],
        bump

    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [b"nft_vault", nft_mint.key().as_ref()],
        token::mint = nft_mint,
        token::authority = vault_authority,
        bump,
    )]
    pub nft_vault: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
        seeds = [b"vault_authority", &id().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stake_account", nft_mint.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,
    pub nft_vault: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"vault_authority", &id().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    pub nft_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: You are not the staker of this NFT")]
    UnauthorizedUnstake,
    #[msg("NFT is not currently staked")]
    NotStaked,
    #[msg("Invalid stake account")]
    InvalidStakeAccount,
}