use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Insufficient funds for vesting")]
    InsufficientFundsToVest,

    #[msg("Invalid vesting type")]
    InvalidVestingType,

    #[msg("Numerical overflow occurred.")]
    NumericalOverflow,

    #[msg("All tokens vested")]
    NoAvailableTokensToVest,

    #[msg("Tokens already claimed")]
    TokensAlreadyClaimed,

    #[msg("Vesting not started")]
    VestingNotStarted,

    #[msg("Cliff period not passed yet!")]
    CliffPeriodNotOver,

    #[msg("All vested tokens claimed!")]
    VestingContractFullyClaimed,

    #[msg("Vesting contract not active!")]
    VestingNotActive,

    #[msg("Can't claim multiple times!")]
    ClaimMultipleNotAllowed,

    NoTokensToClaim,

    #[msg("Vesting already closed")]
    VestingIsNotActive,

    #[msg("Vesting is not revocable because its not marked as revocable")]
    VestingIsNotRevocable,

    #[msg("No tokens in the vault to transfer")]
    VaultIsEmpty,
}


