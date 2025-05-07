#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4Nvkao9hANr7SLvxd859tWMd4bLKcXKKCv8emkaCFWht");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fees: u16) -> Result<()> {
        ctx.accounts.init(name, fees, &ctx.bumps)?;
        msg!("Market place initialized successfully!");
        Ok(())
    }

    pub fn list_nft(ctx: Context<List>, nft_price: u64) -> Result<()> {
        ctx.accounts.create_listing(nft_price, &ctx.bumps)?;
        msg!("NFT Listed for sale successfully!");
        Ok(())
    }

    pub fn delist_nft(ctx: Context<List>) -> Result<()> {
        msg!("NFT Removed from sale successfully!");
        ctx.accounts.delist_nft()?;
        Ok(())
    }

    pub fn purchase_nft(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol()?;
        ctx.accounts.send_nft()?;
        msg!("NFT Purchased successfully!");
        Ok(())
    }
}
