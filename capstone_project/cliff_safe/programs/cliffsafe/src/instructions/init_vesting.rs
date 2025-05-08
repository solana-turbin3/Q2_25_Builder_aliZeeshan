use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::state::vesting_contract_info::VestingContractInfo;

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct InitializeVesting<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mint::token_program = token_program,
        mint::authority = creator,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + VestingContractInfo::INIT_SPACE,
        seeds = [company_name.as_bytes().as_ref(), creator.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub vesting_contract_info: Box<Account<'info, VestingContractInfo>>,

    #[account(
        init,
        payer = creator,
        seeds = [b"vault", mint.key().as_ref(), creator.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vesting_contract_info,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = creator,
    )]
    pub company_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl<'info> InitializeVesting<'info> {
    pub fn initialize_vesting(&mut self, company_name: String, is_revocable: bool, bump: &InitializeVestingBumps) -> Result<()> {
        let vesting_contract_info = &mut self.vesting_contract_info;
        vesting_contract_info.set_inner(VestingContractInfo {
            creator: self.creator.key(),
            company_name,
            mint: self.mint.key(),
            total_vested_tokens: 0,
            total_available_tokens: 0,
            total_locked_tokens: 0,
            total_claimed_tokens: 0,
            vault_account: self.vault.key(),
            created_at: Clock::get()?.unix_timestamp,
            vault_bump: bump.vault,
            bump: bump.vesting_contract_info,
            is_active: true,
            revocable: is_revocable,
            is_revoked: false,
            fully_claimed: false,
        });

        msg!("Vesting contract initialized for company: {}", vesting_contract_info.company_name);
        msg!("Vesting contract info: {:?}", vesting_contract_info);

        Ok(())
    }
}