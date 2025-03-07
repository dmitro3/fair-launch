use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Only admin can call this function")]
    OnlyAdmin,

    #[msg("Only DAO can call this function")]
    OnlyDAO,

    #[msg("Invalid Fee")]
    InvalidFee,

    #[msg("Invalid Quorum")]
    InvalidQuorum,

    #[msg("DAO already activated")]
    DAOAlreadyActivated,

    #[msg("Overflow or underflow occured")]
    OverFlowUnderFlowOccured,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Not enough SOL in vault")]
    NotEnoughSolInVault,

    #[msg("Invalid bonding curve type")]
    InvalidBondingCurveType,

}
