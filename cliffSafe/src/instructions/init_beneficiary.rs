use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::state::vesting_record_info::*;
use crate::state::vesting_contract_info::*;

use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(vesting_type: u8, cliff_period: i64, vesting_start_time: i64, vesting_end_time: i64, company_name: String)]
pub struct InitializeBeneficiary<'info> {
    /// The company or vesting contract owner, who is funding and initializing the beneficiary.
    #[account(mut)]
    pub creator: Signer<'info>,

    /// The wallet receiving the vesting tokens.
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
    pub fn init_beneficiary(&mut self, vesting_type: u8, cliff_period: i64, vesting_start_time: i64, vesting_end_time: i64, company_name: String) -> Result<()>  {
        msg!("Initializing beneficiary...");

        let vesting_record_info = &mut self.vesting_record_info;

        let vesting_type_enum = match vesting_type {
            0 => VestingType::Cliff,
            1 => VestingType::Linear,
            _ => return Err(ErrorCode::InvalidVestingType.into()),
        };

        let vesting_record_data = Box::new(VestingRecordInfo {
            beneficiary: self.beneficiary.key(),
            mint: self.mint.key(),
            total_vested_tokens: self.vesting_contract_info.total_locked_tokens,
            total_claimed_tokens_by_beneficiary: 0,
            vesting_type: vesting_type_enum,
            cliff_period,
            vesting_start_time,
            vesting_end_time,
            vault_bump: self.vesting_contract_info.vault_bump,
            vesting_contract_bump: self.vesting_contract_info.bump,
            beneficiary_ata: self.beneficiary_ata.key(),
            has_claimed: false,
            is_active: true,
        });

        vesting_record_info.set_inner(*vesting_record_data);
        

        msg!("Beneficiary Account Info : ${:?}", vesting_record_info);

        Ok(())
    }
        
}
