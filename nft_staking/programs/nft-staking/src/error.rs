use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Max Stake Reached")]
    MaxStakeReached,
    #[msg("Freeze Period Not Passed")]
    FreezePeriodNotPassed,
}
