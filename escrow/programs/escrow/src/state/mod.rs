use anchor_lang::prelude::*;


#[derive(InitSpace)]
#[account]
pub struct Escrow {
    pub seed: u64,      // identifier for the escrow
    pub maker: Pubkey,
    // pub taker: Pubkey, 
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub give: u64,
    pub receive: u64,
    pub bump: u8,
}