pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2Baxgb25EcJqLHfDycTfpfKrz7cPJjLNAsx9DEsQxyoj");

#[program]
pub mod cliff_safe { 
    use super::*;

    pub fn initialize_vesting(
        ctx: Context<InitializeVesting>, 
        company_name: String, 
        is_revocable: bool
    ) -> Result<()>  {
        ctx.accounts.initialize_vesting(company_name, is_revocable, &ctx.bumps)
    }

    pub fn initialize_beneficiary(
        ctx: Context<InitializeBeneficiary>, 
        company_name: String, 
        vesting_type: u8, 
        cliff_period: i64, 
        start_time: i64, 
        end_time: i64, 
        vesting_amount: u64
    ) -> Result<()> {
        ctx.accounts.initialize_beneficiary(
            company_name,
            vesting_type, 
            cliff_period, 
            start_time, 
            end_time, 
            vesting_amount
        )
    }

    pub fn initialize_beneficiary_ata(ctx: Context<InitBeneficiaryAta>) -> Result<()> {
        ctx.accounts.init_beneficiary_ata()
    } 

    pub fn deposit_tokens(ctx: Context<DepositTokens>, deposit_amount: u64, company_name: String) -> Result<()> {
        ctx.accounts.deposit_tokens(deposit_amount, company_name)
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, mint_amount: u64, company_name: String) -> Result<()> {
        ctx.accounts.mint_tokens(mint_amount, company_name)
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>, company_name: String) -> Result<()> {
        ctx.accounts.claim_tokens(company_name)
    }

    pub fn revoke_vesting(ctx: Context<RevokeVesting>, company_name: String) -> Result<()> {
        ctx.accounts.revoke_vesting(company_name)
    }
}
