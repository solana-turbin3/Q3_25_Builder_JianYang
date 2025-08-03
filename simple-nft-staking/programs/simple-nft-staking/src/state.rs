use anchor_lang::prelude::*;

// StakeConfig is the global config for a particular staking program
// each deployment of the program initializes a global config using StakeConfig
#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub authority: Pubkey,          // program admin - authoritative control
    pub collection_mint: Pubkey,
    pub points_per_second: u64,
    pub max_stake_per_user: u8,     // max no. of NTFs a single user can stake
    pub freeze_period: i64,         // minimum time before the user can unstake
    pub bump: u8,
}

// UserAccount is created once per user
// it takes care of the metrics of each user
#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub staker: Pubkey,
    pub accumulated_points: u64,
    pub total_staked: u8,
    pub last_claim: i64,
    pub bump: u8,
}

// StakeAccount is created once per user per nft
#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub staker: Pubkey,
    pub nft_mint: Pubkey,
    pub staked_at: i64,
    pub last_updated: i64,
    pub staker_stats: Pubkey,
}
