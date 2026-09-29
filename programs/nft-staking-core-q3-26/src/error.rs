use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("NFT owner key mismatch")]
    InvalidOwner,
    #[msg("Invalid update authority")]
    InvalidAuthority,
    #[msg("NFT already staked")]
    AlreadyStaked,
    #[msg("NFT not staked")]
    NotStaked,
    #[msg("Invalid timestamp value")]
    InvalidTimestamp,
    #[msg("NFT freeze period not elapsed")]
    FreezePeriodNotElapsed,
    #[msg("Overflow")]
    Overflow,
    #[msg("Invalid number")]
    InvalidNumber,
    #[msg("Already updated")]
    AlreadyUpdated,    
}