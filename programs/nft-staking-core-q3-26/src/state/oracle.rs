use anchor_lang::prelude::*;

use crate::constants::{BOUNDARY_TOLERANCE, SECONDS_PER_DAY, TRANSFER_WINDOW_END, TRANSFER_WINDOW_START};

#[account]
#[derive(InitSpace)]
pub struct Oracle {
    pub validation: OracleValidation,
    pub collection: Pubkey,
    pub last_updated: i64,
    pub bump: u8,
    pub vault_bump: u8,
}

impl Oracle {
    pub fn is_allowed_transfer(unix_timestamp: i64) -> bool {
        let seconds_of_day = unix_timestamp.rem_euclid(SECONDS_PER_DAY as i64) as u32;
        
        (TRANSFER_WINDOW_START..TRANSFER_WINDOW_END).contains(&seconds_of_day)
    }

    pub fn is_boundary_time(unix_timestamp: i64) -> bool {
        let seconds_since_midnight = unix_timestamp.rem_euclid(SECONDS_PER_DAY as i64);
        
        let start = TRANSFER_WINDOW_START as i64;
        let end = TRANSFER_WINDOW_END as i64;
        let tolerance = BOUNDARY_TOLERANCE as i64;

        (start..start.saturating_add(tolerance)).contains(&seconds_since_midnight)
            || (end..end.saturating_add(tolerance)).contains(&seconds_since_midnight)
    } 

    pub fn transfer_validation(unix_timestamp: i64) -> ExternalValidationResult {
        if Self::is_allowed_transfer(unix_timestamp) {
            ExternalValidationResult::Approved
        } else {
            ExternalValidationResult::Rejected
        }
    }

    pub fn make_validation(transfer_validation: ExternalValidationResult) -> OracleValidation {
        OracleValidation::V1 {
            transfer: transfer_validation,
            create: ExternalValidationResult::Pass,
            burn: ExternalValidationResult::Pass,
            update: ExternalValidationResult::Pass,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Eq, PartialEq)]
pub enum OracleValidation {
    Uninitialized,
    V1 {
        create: ExternalValidationResult,
        transfer: ExternalValidationResult,
        burn: ExternalValidationResult,
        update: ExternalValidationResult,
    },
}

impl Space for OracleValidation {
    const INIT_SPACE: usize = 1 + 4 * ExternalValidationResult::INIT_SPACE;
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExternalValidationResult {
    Approved,
    Rejected,
    Pass,
}

impl Space for ExternalValidationResult {
    const INIT_SPACE: usize = 1;
}

impl ExternalValidationResult {
    pub fn toggle(self) -> Self {
        match self {
            Self::Approved => Self::Rejected,
            Self::Rejected => Self::Approved,
            Self::Pass => Self::Pass,
        }
    }
}
