use anchor_lang::prelude::*;

#[constant]
pub const SECONDS_PER_DAY: i64 = 86400;

#[constant]
pub const TRANSFER_WINDOW_START: u32 = 9 * 3600;

#[constant]
pub const TRANSFER_WINDOW_END: u32 = 17 * 3600;

#[constant]
pub const BOUNDARY_TOLERANCE: i64 = 60;

#[constant]
pub const CRANK_REWARD_LAMPORTS: u64 = 10_000_000;

#[constant]
pub const VAULT_INITIAL_FUND: u64 = 100_000_000;
