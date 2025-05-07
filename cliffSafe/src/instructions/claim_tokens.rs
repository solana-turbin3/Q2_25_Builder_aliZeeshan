use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::state::vesting_contract_info::VestingContractInfo;
use crate::state::vesting_record_info::VestingRecordInfo;
use crate::{vesting_contract_info::*, VestingType};

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
        bump = vesting_contract_info.vault_bump,
        constraint = vesting_contract_info.creator == creator.key(),
    )]
    pub vesting_contract_info: Account<'info, VestingContractInfo>,

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
    pub fn process_claim_tokens(&mut self, company_name: String) -> Result<()> {
        let vesting_contract_info = &mut self.vesting_contract_info;
        let vesting_record_info = &mut self.vesting_record_info;
        let vesting_type = vesting_record_info.vesting_type;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time < vesting_record_info.vesting_start_time {
            return Err(ErrorCode::VestingNotStarted.into());
        }

        let total_vested = vesting_record_info.total_vested_tokens;
        let total_claimed = vesting_record_info.total_claimed_tokens_by_beneficiary;
        let mut claimable_tokens: u64 = 0;

        let elapsed_time = current_time - vesting_record_info.vesting_start_time; // 500 1000

        match vesting_type {
            VestingType::Cliff => {
                let cliff_time = vesting_record_info.cliff_period;

                if elapsed_time < cliff_time {
                    return Err(ErrorCode::CliffPeriodNotReached.into());
                }

                if vesting_record_info.has_claimed == true {
                    return Err(ErrorCode::TokensAlreadyClaimed.into());
                };

                if total_claimed == total_vested {
                    return Err(ErrorCode::TokensAlreadyClaimed.into());
                }

                claimable_tokens = ((elapsed_time.saturating_sub(cliff_time)) * total_vested as i64 / (vesting_record_info.vesting_end_time - vesting_record_info.vesting_start_time)) as u64;
            },
            VestingType::Linear => {
                if total_claimed == total_vested {
                    return Err(ErrorCode::TokensAlreadyClaimed.into());
                }

                claimable_tokens = (total_vested * (elapsed_time) as u64) / (vesting_record_info.vesting_end_time - vesting_record_info.vesting_start_time) as u64; 
            }
        }

        vesting_contract_info.total_claimed_tokens += claimable_tokens;

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.beneficiary_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.vesting_contract_info.to_account_info(),
        };

        let company_name = self.vesting_contract_info.company_name.clone();
        let creator = self.creator.key();
        let mint = self.mint.key();

        let signer_seeds: &[&[&[u8]]] = &[&[
            company_name.as_ref(),
            creator.as_ref(),
            mint.as_ref(),
            &[self.vesting_contract_info.bump]
        ]];
        let cpi_program = self.token_program.to_account_info();

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, claimable_tokens, self.mint.decimals)?;

        msg!("Claimed {} tokens by the beneficiary", claimable_tokens);

        vesting_record_info.total_claimed_tokens_by_beneficiary += claimable_tokens;

        if vesting_record_info.total_claimed_tokens_by_beneficiary == vesting_record_info.total_vested_tokens {
            vesting_record_info.has_claimed = true;
        }

        msg!("Beneficiary has claimed a total of {} tokens", vesting_record_info.total_claimed_tokens_by_beneficiary);


        Ok(())
    }
}
