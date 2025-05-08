use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct VestingContractInfo {
    pub creator: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub mint: Pubkey,
    pub total_vested_tokens: u64, //The number of tokens that the company has deposited in the vault
    pub total_available_tokens: u64, //The number of tokens that are currently available for distribution
    //Increases when the company initializes a new beneficiary
    //Company also specifies the number of tokens that the beneficiary will receive ex : 1000 tokens
    pub total_locked_tokens: u64, // Total tokens locked in vesting contracts(Total Vested Tokens)
    pub total_claimed_tokens: u64, 
    pub vault_account: Pubkey,
    pub created_at: i64,
    pub vault_bump: u8,
    pub bump: u8,
    pub is_active: bool,
    pub revocable: bool,
    pub is_revoked: bool,
    pub fully_claimed: bool,
}