pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EkZbYtVUhehzFvQrMrrv5U5RMH56e37Dk26vzXgk3Kvd");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn init_config(ctx: Context<InitializeConfig>, point_per_stake: u64, max_stake: u64, freeze_period: u64) -> Result<()> {
        ctx.accounts.initialize_config(point_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        msg!("Initialized config account successfully!");
        Ok(())
    }

    pub fn init_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)?;
        msg!("Uset account initialized successfully!");
        Ok(())
    }

    pub fn stake_nft(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        msg!("Nft Staked Successfully!");
        Ok(())
    }
}

