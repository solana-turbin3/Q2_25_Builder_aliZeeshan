use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::state::vesting_contract_info::VestingContractInfo;

#[derive(Accounts)]
#[instruction(deposit_amount: u64, company_name: String)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mint::token_program = token_program,
        mint::authority = creator,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [company_name.as_bytes().as_ref(), creator.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub vesting_contract_info: Box<Account<'info, VestingContractInfo>>,

    #[account(mut,
        seeds = [b"vault", mint.key().as_ref(), creator.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vesting_contract_info,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = creator,
    )]
    pub company_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> DepositTokens<'info> {
    pub fn deposit_tokens(&mut self, deposit_amount: u64, company_name: String) -> Result<()> {
        let vesting_contract_info = &mut self.vesting_contract_info;
        vesting_contract_info.total_vested_tokens += deposit_amount;
        vesting_contract_info.total_available_tokens += deposit_amount;

        let cpi_accounts = TransferChecked {
            from: self.company_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.creator.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, deposit_amount, self.mint.decimals)?;
        msg!("Deposited {} tokens to the vault", deposit_amount);

        Ok(())
    }
}