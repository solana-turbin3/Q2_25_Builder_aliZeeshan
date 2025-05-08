use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct VestingRecordInfo {
    pub beneficiary: Pubkey,
    pub mint: Pubkey,
    pub total_vested_tokens: u64,
    pub total_claimed_tokens_by_beneficiary: u64,
    pub vesting_type: VestingType,
    pub cliff_period: i64,
    pub vault_bump: u8,
    pub vesting_contract_info_bump: u8,
    pub bump: u8,
    pub beneficiary_ata: Pubkey,
    pub has_claimed: bool,
    pub claim_multiple: bool,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, InitSpace)]
pub enum VestingType {
    CliffVesting,
    LinearVesting,
}