use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{close_account, CloseAccount}, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

use crate::{Escrow};


// Take context 
// This will take care of 3 operations
// 1. Taker to Maker (without involving the escrow)
// 2. Release of funds from Vault to Taker
// 3. Cleanup by closing of vault
// But how will the escrow be closed?
#[derive(Accounts)]
pub struct Take<'info> {
    // mut since taker will be paying for these instructions
    #[account(mut)]
    pub taker: Signer<'info>,
    // `maker` doesn't need to sign so we'll just use it as a `SystemAccount`
    // it's only being used as a user wallet
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,     // `init_if_needed` bc it may or may not exist already
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,    // maker is authority bc it's maker's ata for `mint_b`
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    
    // validating the seeds and bump for the escrow passed in 
    #[account(
        mut,
        seeds = [b"escrow", escrow.maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,     // bump is being taken from the escrow data
        close = maker           // this escrow will be closed at the end of the intruction
                                // rent will be sent to `maker` since they initialized it
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
// escrow account closed
impl <'info> Take<'info> {
    // helper function for taker sending token_b -> maker
    pub fn taker_to_maker(&mut self) -> Result<()> {
        // TransferChecked does stricter validation than Transfer
        let transfer_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
        };
        // `CpiContext::new()` used because a regular Non-PDA account is the Signer
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        // taker to maker token_b transfer happens here
        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)
    }
    
    // escrow releases token_a from vault to taker
    pub fn release_and_close_vault(&mut self) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        // `signer_seeds` required since Signer is a PDA (escrow PDA) 
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
            ]];
            let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), transfer_accounts, &signer_seeds);
            // vault to taker transfer of token_a
            transfer_checked(cpi_ctx, self.escrow.give, self.mint_a.decimals)?;
            
            let close_accounts = CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.maker.to_account_info(),
                authority: self.escrow.to_account_info(),
            };
            
        // vault account closed and rent transferred to destination aka `maker`
        let close_cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, &signer_seeds);
        close_account(close_cpi_ctx)
    }
        

}