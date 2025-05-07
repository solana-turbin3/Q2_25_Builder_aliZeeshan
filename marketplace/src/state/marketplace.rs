use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fees: u16,
    pub bump: u8,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    #[max_len(50)]
    pub name: String,
}

// impl Space for Marketplace {
//     const INIT_SPACE: usize = 8 + 32 + 1 + 1 + 1 + (4 + 32);
// }
