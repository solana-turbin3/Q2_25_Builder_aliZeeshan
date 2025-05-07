use anchor_lang::prelude::*;
use anchor_spl::token::*;
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct InitializeBeneficiaryATA<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = beneficiary,
        associated_token::mint = mint,
        associated_token::authority = beneficiary,
    )]
    pub beneficiary_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializeBeneficiaryATA<'info> {
    pub fn create_beneficiary_ata(&mut self) -> Result<()> {
        msg!("Beneficiary ATA initialized successfully");
        Ok(())
    }
}
