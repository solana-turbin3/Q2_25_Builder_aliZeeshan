use anchor_lang::prelude::*;
use anchor_spl::associated_token::{AssociatedToken};
use anchor_spl::token::{Mint, Token, TokenAccount};

mod escrow;

#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: Account<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_a_ata: Account<'info, TokenAccount>,

    pub escrow: Account<'info, Escrow>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub  associated_token_program: Program<'info, AssociatedToken>,
}