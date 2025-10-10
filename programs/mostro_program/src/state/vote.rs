#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    /// The proposal this vote belongs to
    pub proposal: Pubkey,

    /// The voter account
    pub voter: Pubkey,

    /// Vote choice (true = yes, false = no)
    pub vote_choice: bool,

    /// Voting power of the voter
    pub voting_power: u64,

    /// Bump for PDA
    pub bump: u8,
}

impl Vote {
    /// Compute space needed for Vote account
    pub fn space() -> usize {
        8 + // discriminator
        32 + // proposal Pubkey
        32 + // voter Pubkey
        1 +  // vote_choice (bool)
        8 +  // voting_power
        1 // bump
    }
}
