// `make.rs` contains the logic for the creation of the `Escrow`
// Creator of the `Escrow` is called `Maker`

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::{Escrow};

#[derive(Accounts)]
// this seed will be used for escrow creation (line 49)
#[instruction(seed: u64)]
pub struct Make <'info> {       // Context for escrow creation step
    // Signer has to be mutable since it has to pay for the creation of the Escrow and Vault PDA
    #[account(mut)]
    pub maker: Signer<'info>,

    // 'mint::token_program = token_program' is an Anchor constraint

    // It ensures the `token_program` passed matches the `token_program`
    // that takes care of the tokens of this particular `Mint` - `mint_a`

    // InterfaceAccount is used as we want to support both multiple SPL standards
    // Here for eg. we want to support both SPL mints and Token2022 token mints 
    // so we are not really concerned with the exact  deserialization 
    // of the passed mint_a account
    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,
    
    // `associated_token` is yet again an Anchor constraint
    // `maker_ata_a` has to be mutable as it's going to transfer
    // token_a to the escrow
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,     // This `TokenAccount` import is from anchor_spl::token_interface
    
    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,     // custom struct thus `Account` type

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        // @note - after completing the program change the authority to maker and try to exploit
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,   // deals with token standards thus `InterfaceAccount`
                                                        // `TokenAccount` deals with token operations
    pub token_program: Interface<'info, TokenInterface>,    // `Interface` is kinda analogous to `Program` for known programs
    pub system_program: Program<'info, System>,
    // `AssociatedToken Program` doesn't change with the use of different token standards
    pub associated_token_program: Program<'info, AssociatedToken>,  
}

impl <'info> Make<'info> {
    // helper function to initialize escrow
    pub fn init_escrow(
        &mut self, 
        seed: u64, 
        give: u64, 
        receive: u64, 
        bumps: &MakeBumps
    ) -> Result<()> {
        self.escrow.set_inner(Escrow { seed, maker: self.maker.key(), mint_a: self.mint_a.key(), mint_b: self.mint_b.key(), give: give, receive: receive, bump: bumps.escrow });
        Ok(())
    }

    // helper function to deposit from the maker into the vault
    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            mint: self.mint_a.to_account_info(),
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)
    }
}