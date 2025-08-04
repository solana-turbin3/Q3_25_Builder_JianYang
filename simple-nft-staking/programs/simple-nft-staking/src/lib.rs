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

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_second: u64,
        collection_mint: Pubkey,
        max_stake_per_user: u8,
        freezing_period: i64,
    ) -> Result<()> {
        let stake_config = &mut ctx.accounts.stake_config;
        stake_config.authority = ctx.accounts.authority.key();
        stake_config.collection_mint = collection_mint;
        stake_config.points_per_second = points_per_second;
        stake_config.max_stake_per_user = max_stake_per_user;
        stake_config.freeze_period = freezing_period;
        stake_config.bump = ctx.bumps.stake_config;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.staker = ctx.accounts.payer.key();
        user_account.accumulated_points = 0;
        user_account.last_claim = Clock::get()?.unix_timestamp;
        user_account.total_staked = 0;
        user_account.bump = ctx.bumps.user_account;
        Ok(())
    }
    pub fn stake(ctx: Context<Stake>, nft_mint: Pubkey) -> Result<()> {
        let stake_config = &mut ctx.accounts.stake_config;
        let stake_account = &mut ctx.accounts.stake_account;
        stake_account.staker = ctx.accounts.payer.key();
        stake_account.staked_at = Clock::get()?.unix_timestamp;
        stake_account.nft_mint = nft_mint;
        stake_account.last_updated = Clock::get()?.unix_timestamp;
        stake_account.staker_stats = ctx.accounts.user_account.key();
        
        let user_account = &mut ctx.accounts.user_account;
        // CHECK:: User staking limit has not been crossed
        require!(
            user_account.total_staked < stake_config.max_stake_per_user,
            ErrorCode::MaxStakeExceeded
        );

        // CHECK:: User owns the NFT
        require!(
            ctx.accounts.user_nft_account.owner == ctx.accounts.payer.key(),
            ErrorCode::NotNFTOwner
        );

        // CHECK:: User is staking only one NFT per instruction
        require!(
            ctx.accounts.user_nft_account.amount == 1,
            ErrorCode::InvalidNFTAmount
        );
        
        // update rewards for the existing staked nfts
        if user_account.total_staked > 0 {
            let time_diff = Clock::get()?.unix_timestamp - user_account.last_claim;
            let pending_points = (time_diff as u64)
            .checked_mul(stake_config.points_per_second)
            .ok_or(ErrorCode::PendingPointsArithmeticOverflow)?
            .checked_mul(user_account.total_staked as u64)
            .ok_or(ErrorCode::PendingPointsArithmeticOverflow)?;

            user_account.accumulated_points = user_account.accumulated_points
                .checked_add(pending_points)
                .ok_or(ErrorCode::PendingPointsArithmeticOverflow)?;
        }
    
        // user stats update
        user_account.total_staked += 1;
        // update the timestamp
        user_account.last_claim = Clock::get()?.unix_timestamp;

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
        
        // user stats update
        let user_account = &mut ctx.accounts.user_account;
        user_account.total_staked -= 1;

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

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let current_timestamp = Clock::get()?.unix_timestamp;
        let user_account = &mut ctx.accounts.user_account;

        // time elapsed since the last claim
        let time_diff = current_timestamp - user_account.last_claim;

        let total_new_points = (time_diff as u64)
        .checked_mul(ctx.accounts.stake_config.points_per_second)
        .ok_or(ErrorCode::RewardsOverflow)?
        .checked_mul(user_account.total_staked as u64)
        .ok_or(ErrorCode::RewardsOverflow)?;

        // update accumulated points
        user_account.accumulated_points = user_account.accumulated_points
            .checked_add(total_new_points as u64)
            .ok_or(ErrorCode::RewardsOverflow)?;
        
        // updating last claim time
        user_account.last_claim = current_timestamp;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user_account", payer.key.as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        seeds = [b"stake_config"],
        bump
    )]
    pub stake_config: Account<'info, StakeConfig>,

}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + StakeConfig::INIT_SPACE,
        seeds = [b"stake_config"],
        bump
    )]
    pub stake_config: Account<'info, StakeConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [b"user_account", payer.key.as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
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
    /// CHECK: this is a PDA derived from program ID, used as token authority for vaults
    #[account(
        seeds = [b"vault_authority", &id().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [b"user_account", payer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(seeds = [b"stake_config"], bump = stake_config.bump)]
    pub stake_config: Account<'info, StakeConfig>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"stake_account", nft_mint.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,
    pub nft_vault: Account<'info, TokenAccount>,
    /// CHECK: this is a PDA derived from program ID, used as token authority for vaults
    #[account(
        seeds = [b"vault_authority", &id().as_ref()],
        bump
    )]
    pub vault_authority: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [b"user_account", payer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
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
    #[msg("Mismatched NFT Mint")]
    InvalidCollection,
    #[msg("Staker's Max Stake Amount Exceeded")]
    MaxStakeExceeded,
    #[msg("Invalid NFT Amount")]
    InvalidNFTAmount,
    #[msg("Staker Is Not NFT Owner")]
    NotNFTOwner,
    #[msg("Arithmetic Overflow In Rewards Calculation")]
    RewardsOverflow,
    #[msg("Arithmetic Overflow In Pending Rewards Calculation")]
    PendingPointsArithmeticOverflow,
} 