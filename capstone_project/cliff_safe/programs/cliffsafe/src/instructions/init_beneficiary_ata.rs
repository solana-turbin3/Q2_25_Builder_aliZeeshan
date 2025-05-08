use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct InitBeneficiaryAta<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = beneficiary,
        associated_token::mint = mint,
        associated_token::authority = beneficiary,
    )]
    pub beneficiary_ata: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitBeneficiaryAta<'info> {
    pub fn init_beneficiary_ata(&mut self) -> Result<()> {
        msg!("Beneficiary ATA initialized");
        Ok(())
    }
}