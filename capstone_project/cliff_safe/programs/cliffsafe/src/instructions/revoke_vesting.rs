use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

use crate::state::vesting_contract_info::VestingContractInfo;

use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct RevokeVesting<'info> {
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
        associated_token::authority = creator,
    )]
    pub company_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> RevokeVesting<'info> {
    pub fn revoke_vesting(&mut self, company_name: String) -> Result<()> {

        if self.vesting_contract_info.is_active == false && self.vesting_contract_info.is_revoked == true {
            return Err(ErrorCode::VestingIsNotActive.into());
        };

        if self.vesting_contract_info.revocable == false {
            return Err(ErrorCode::VestingIsNotRevocable.into());
        };

        let transfer_amount = self.vault.amount;
        require!(transfer_amount > 0, ErrorCode::VaultIsEmpty);

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.company_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.vesting_contract_info.to_account_info(),
        };

        let creator = self.creator.key();
        let mint = self.mint.key();

        let signer_seeds: &[&[&[u8]]] = &[
            &[
              company_name.as_bytes().as_ref(),
              creator.as_ref(),
              mint.as_ref(),
              &[self.vesting_contract_info.bump]
            ]
        ];

        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer_seeds);

        transfer_checked(cpi_context, transfer_amount, self.mint.decimals)?;

        self.vesting_contract_info.is_active = false;
        self.vesting_contract_info.is_revoked = true;

        msg!("Vesting Revoked Successfully!");
        msg!("{} tokens transferred back from vault to company ATA", transfer_amount);

        Ok(())
    }
}