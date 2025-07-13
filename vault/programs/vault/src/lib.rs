#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("5xjzBZvz6cTaCQDRy62jerJMk3QCMHoie3EfXTtVSqtr");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user : Signer<'info>,
    #[account(init, payer = user, space = 8 + 1 + 1, seeds = [b"vault_state", user.key().as_ref()], bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds = [b"vault", user.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl <'info> Initialize<'info> {
    pub fn initialize (&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [b"vault_state", user.key().as_ref()], bump = vault_state.state_bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut, seeds=[b"vault", user.key().as_ref()], bump = vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}
impl <'info> Payment <'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{ from: self.user.to_account_info(), to: self.vault.to_account_info()};
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // transfer(&self.user.key(), &self.vault.key(), amount);
        transfer(cpi_ctx, amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{ from: self.vault.to_account_info(), to: self.user.to_account_info(),};
        let seeds = [b"vault", self.user.key.as_ref(), &[self.vault_state.vault_bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(seeds = [b"vault_state", user.key().as_ref()], bump = vault_state.state_bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut, seeds = [b"vault", user.key().as_ref()], bump = vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl <'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{ from: self.vault.to_account_info(), to: self.user.to_account_info()};
        let seeds = [b"vault", self.user.key.as_ref(), &[self.vault_state.vault_bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        let lamports = self.vault.lamports();
        transfer(cpi_ctx, lamports)
    }
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

