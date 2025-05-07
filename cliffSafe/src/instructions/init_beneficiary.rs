use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::state::vesting_record_info::{VestingRecordInfo, VestingType};
use crate::state::vesting_contract_info::VestingContractInfo;

use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(company_name: String, vesting_type: u8, cliff_period: i64, start_time: i64, end_time: i64, vesting_amount: u64)]
pub struct InitializeBeneficiary<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub beneficiary: SystemAccount<'info>,

    #[account(
        mint::token_program = token_program,
        mint::authority = creator,
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [company_name.as_bytes().as_ref(), creator.key().as_ref(), mint.key().as_ref()],
        bump,
        constraint = vesting_contract_info.creator == creator.key(),
    )] 
    pub vesting_contract_info: Box<Account<'info, VestingContractInfo>>,

    #[account(
        init,
        payer = creator,
        space = 8 + VestingRecordInfo::INIT_SPACE,
        seeds = [vesting_contract_info.key().as_ref(), beneficiary.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub vesting_record_info: Box<Account<'info, VestingRecordInfo>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = beneficiary,
    )]
    pub beneficiary_ata: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeBeneficiary<'info> {
    pub fn initialize_beneficiary(&mut self, company_name: String, vesting_type: u8, cliff_period: i64, start_time: i64, end_time: i64, vesting_amount: u64) -> Result<()> {
        let vesting_record_info = &mut self.vesting_record_info;
        let vesting_contract_info = &mut self.vesting_contract_info;

        if vesting_contract_info.total_available_tokens < vesting_amount && vesting_contract_info.total_available_tokens == 0 && vesting_contract_info.total_locked_tokens == vesting_contract_info.total_vested_tokens {
            return Err(ErrorCode::NoAvailableTokensToVest.into());
        }

        let vesting_type = match vesting_type {
            0 => VestingType::CliffVesting,
            1 => VestingType::LinearVesting,
            _ => return Err(ErrorCode::InvalidVestingType.into()),
        };

        let claim_multiple = match vesting_type {
            VestingType::CliffVesting => false,
            VestingType::LinearVesting => true,
        };

        vesting_record_info.set_inner(VestingRecordInfo{
            beneficiary: self.beneficiary.key(),
            mint: self.mint.key(),
            total_vested_tokens: vesting_amount,
            total_claimed_tokens_by_beneficiary: 0,
            vesting_type,
            cliff_period,
            vault_bump: vesting_contract_info.vault_bump,
            vesting_contract_info_bump: vesting_contract_info.bump,
            bump: vesting_record_info.bump,
            beneficiary_ata: self.beneficiary_ata.key(),
            has_claimed: false,
            claim_multiple,
            start_time,
            end_time,
        });

        vesting_contract_info.total_locked_tokens = vesting_contract_info.total_locked_tokens
            .checked_add(vesting_amount)
            .ok_or(ErrorCode::NumericalOverflow)?;

        vesting_contract_info.total_available_tokens = vesting_contract_info.total_available_tokens
            .checked_sub(vesting_amount)
            .ok_or(ErrorCode::NumericalOverflow)?;

        msg!("Beneficiary record info : {:?}", vesting_record_info);

        Ok(())
    }
}
