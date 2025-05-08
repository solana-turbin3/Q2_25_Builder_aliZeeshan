use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::state::vesting_record_info::{VestingRecordInfo, VestingType};
use crate::state::vesting_contract_info::VestingContractInfo;

use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    pub creator: SystemAccount<'info>,

    #[account(
        mint::token_program = token_program,
        mint::authority = creator,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [company_name.as_bytes().as_ref(), creator.key().as_ref(), mint.key().as_ref()],
        bump,
        constraint = vesting_contract_info.creator == creator.key(),
    )] 
    pub vesting_contract_info: Box<Account<'info, VestingContractInfo>>,

    #[account(
        mut,
        seeds = [vesting_contract_info.key().as_ref(), beneficiary.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub vesting_record_info: Account<'info, VestingRecordInfo>,

    #[account(
        mut,
        seeds = [b"vault", mint.key().as_ref(), creator.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vesting_contract_info,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = beneficiary,
    )]
    pub beneficiary_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> ClaimTokens<'info> {

    pub fn claim_tokens(&mut self, company_name: String) -> Result<()> {
        let vesting_contract_info = &mut self.vesting_contract_info;
        let vesting_record_info = &mut self.vesting_record_info;
        let vesting_type = vesting_record_info.vesting_type;
        let current_time = Clock::get()?.unix_timestamp;
    
        if current_time < vesting_record_info.start_time {
            return Err(ErrorCode::VestingNotStarted.into());
        }
    
        if vesting_contract_info.is_active == false {
            return Err(ErrorCode::VestingNotActive.into());
        }
    
        let total_vested = vesting_record_info.total_vested_tokens;
        let total_claimed = vesting_record_info.total_claimed_tokens_by_beneficiary;
        let mut claimable_tokens: u64 = 0;
    
        match vesting_type {
            VestingType::CliffVesting => {
                let unlock_time = vesting_record_info.start_time
                    .checked_add(vesting_record_info.cliff_period)
                    .ok_or(ErrorCode::NumericalOverflow)?;
    
                if current_time < unlock_time {
                    return Err(ErrorCode::CliffPeriodNotOver.into());
                }
    
                if vesting_record_info.has_claimed {
                    return Err(ErrorCode::TokensAlreadyClaimed.into());
                }
    
                claimable_tokens = total_vested;
            },
            VestingType::LinearVesting => {
                if total_claimed >= total_vested {
                    return Err(ErrorCode::TokensAlreadyClaimed.into());
                }

                let total_duration = vesting_record_info.end_time
                    .checked_sub(vesting_record_info.start_time)
                    .ok_or(ErrorCode::NumericalOverflow)?;
    
                let elapsed_time = current_time
                    .checked_sub(vesting_record_info.start_time)
                    .ok_or(ErrorCode::NumericalOverflow)?;
    
                let vested_so_far = if current_time >= vesting_record_info.end_time {
                    total_vested
                } else {
                    (total_vested * elapsed_time as u64) / total_duration as u64
                };
    
                claimable_tokens = vested_so_far
                    .checked_sub(total_claimed)
                    .ok_or(ErrorCode::NumericalOverflow)?;
    
                if claimable_tokens == 0 {
                    return Err(ErrorCode::NoTokensToClaim.into());
                }
            }
        };
        
        let company_name = vesting_contract_info.company_name.clone();
        let creator = self.creator.key();
        let mint = self.mint.key();

        let signer_seeds: &[&[&[u8]]] = &[&[
            company_name.as_ref(),
            creator.as_ref(),
            mint.as_ref(),
            &[vesting_contract_info.bump]
        ]];
    
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.beneficiary_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: vesting_contract_info.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    
        transfer_checked(cpi_ctx, claimable_tokens, self.mint.decimals)?;
    
        vesting_record_info.total_claimed_tokens_by_beneficiary = vesting_record_info
            .total_claimed_tokens_by_beneficiary
            .checked_add(claimable_tokens)
            .ok_or(ErrorCode::NumericalOverflow)?;
    
        vesting_contract_info.total_claimed_tokens = vesting_contract_info
            .total_claimed_tokens
            .checked_add(claimable_tokens)
            .ok_or(ErrorCode::NumericalOverflow)?;
    
        if vesting_type == VestingType::CliffVesting {
            vesting_record_info.has_claimed = true;
        }
    
        if vesting_contract_info.total_claimed_tokens >= vesting_contract_info.total_vested_tokens {
            vesting_contract_info.is_active = false;
            vesting_contract_info.fully_claimed = true;
        }
    
        msg!("Claimed {} tokens by the beneficiary", claimable_tokens);

        msg!("Vesting record info: {:?}", vesting_record_info);
        msg!("Vesting contract info: {:?}", vesting_contract_info);
        Ok(())
    }
    
}
