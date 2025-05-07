use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{self, transfer, transfer_checked, CloseAccount, Token, Transfer, TransferChecked},
    token_interface::{Mint, TokenAccount}
};

use crate::state::{Listing, Marketplace};


#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        seeds = [b"marketplace", makerplace.name.as_str().as_bytes()],
        bump = makerplace.bump,
    )]
    pub makerplace: Account<'info, Marketplace>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = reward_mint,
        associated_token::authority = taker,
    )]
    pub taker_rewards_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"rewards", makerplace.key().as_ref()],
        bump = makerplace.rewards_bump,
        mint::decimals = 6,
        mint::authority = makerplace
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [makerplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [b"treasury", makerplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub colletion_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol(&mut self) -> Result<()> {

        let marketplace_fee = (self.makerplace.fees as u64).checked_mul(self.listing.price).unwrap().checked_div(10000_u64).unwrap();

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        let amount = self.listing.price.checked_sub(marketplace_fee).unwrap();

        transfer(cpi_context, amount)?;

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts2 = Transfer {
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_context2 = CpiContext::new(cpi_program, cpi_accounts2);

        transfer(cpi_context2, marketplace_fee)?;

        Ok(())
    }

    pub fn send_nft(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_context, 1, self.maker_mint.decimals)?;

        let close_account = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), close_account);

        token::close_account(cpi_context)?;

        Ok(())
    }
}