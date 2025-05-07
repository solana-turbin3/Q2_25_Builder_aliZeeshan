#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{*, TransferChecked, transfer_checked};

declare_id!("FpR4hw1qbcCFCLurXr28pfzvYr3eQ4Yrfp6sZWgkRKJw");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        let vault_state = &ctx.accounts.vault_state;
        let signer = &ctx.accounts.signer;
        let vault = &ctx.accounts.vault;

        let signer_key = signer.key();
        let vault_seeds: &[&[&[u8]]]  = &[
            &[
                b"vault",
                signer_key.as_ref(),
                &[vault_state.vault_bump],
            ],
        ];

        let cpi_accounts = CloseAccount {
            account: vault.to_account_info(),
            destination: signer.to_account_info(),
            authority: vault.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            vault_seeds,
        );

        close_account(cpi_context)?;

        msg!("Vault account closed successfully!");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", signer.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        init,
        payer = signer,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vault,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump = vault_state.vault_bump,
        token::mint = mint,
        token::authority = vault,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
        close = signer,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump = vault_state.vault_bump,
        close = signer,
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bump: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bump.vault;
        self.vault_state.state_bump = bump.vault_state;
        msg!("Vault account initialized successfully!");
        Ok(())
    }
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.signer.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        msg!("Deposited {} tokens to vault", amount);
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let signer = self.signer.to_account_info();
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", signer.key.as_ref(), &[self.vault_state.vault_bump]]];

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
            authority: self.vault.to_account_info(),
            mint : self.mint.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);

        transfer_checked(cpi_context, amount, self.mint.decimals)?;
        msg!("Withdrew {} tokens from vault", amount);

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}
