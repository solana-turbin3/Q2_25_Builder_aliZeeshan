use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub maker: Pubkey, //who listed the NFT
    pub maker_mint: Pubkey,//Mint of the NFT
    pub price: u64,
    pub bump: u8
}
