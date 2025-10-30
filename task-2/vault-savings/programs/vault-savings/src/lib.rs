#![allow(unexpected_cfgs)]
#[allow(deprecated)]

//use std::io::Result;

use anchor_lang::{accounts::signer, prelude::*, system_program::{Transfer, transfer}};

declare_id!("6Ay6Eqdya4VEb6Y64tEimW8fXJsihAoE1upCNnLuR68V");

#[program]
pub mod vault_savings {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.initialize(amount, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdrawal(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.withdrawal(amount)?;
        Ok(())
    }

    pub fn check_balance(ctx: Context<Operations>) -> Result<()> {
        ctx.accounts.check_balance()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state".as_ref(), user.key().as_ref()],
        bump,
        space = VaultSavings::INIT_SPACE,
    )]
    pub state: Account<'info, VaultSavings>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), state.key().as_ref()],
        bump,
    )]
    pub vault_savings_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> Initialize<'info> {
    pub fn initialize (&mut self, amount: u64, bumps: &InitializeBumps) -> Result<()> {
        self.state.amount = amount;
        self.state.vault_bump = bumps.vault_savings_account;
        self.state.state_bump = bumps.state;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Operations <'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"state".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub state: Account<'info, VaultSavings>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), state.key().as_ref()],
        bump,
    )]
    pub vault_savings_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> Operations<'info> {
    pub fn deposit (&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault_savings_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn check_balance(&self) -> Result<()> {
        if self.vault_savings_account.lamports() >= self.state.amount {
            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.vault_savings_account.to_account_info(),
                to: self.user.to_account_info(),
            };

            let seeds = &[
                b"vault".as_ref(), 
                self.state.to_account_info().key.as_ref(),
                &[self.state.vault_bump],
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, self.vault_savings_account.lamports())?;
        }

        Ok(())
    }

    pub fn withdrawal(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault_savings_account.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault".as_ref(), 
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}


/*impl <'info> Withdraw<'info> {
    pub fn withdraw (&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault_savings_account.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault".as_ref(),
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close <'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump,
        close = user
    )]
    pub state: Account<'info, VaultSavings>,
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    pub vault_savings_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault_savings_account.to_account_info(),
            to: self.user.to_account_info(),
        };

        let vault_seeds = &[
            b"vault".as_ref(),
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&vault_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, self.vault_savings_account.lamports());

        msg!("Vault closed and remaining funds transferred to user.");
        Ok(())
    }
}*/


#[account]
pub struct VaultSavings {
    pub amount: u64,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultSavings {
    const INIT_SPACE: usize = 
                            8 // Discriminator
                            + 8 // amount: u64
                            + 1 // vault_bump: u8
                            + 1; // state_bump
}
