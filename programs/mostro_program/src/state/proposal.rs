use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub artist: Pubkey,          // 32 bytes
    pub proposal_id: u64,        // 8 bytes
    pub title: String,           // 4 bytes length prefix + up to MAX_TITLE_LENGTH
    pub number_of_tokens: u64,   // 8 bytes
    pub start_date: i64,         // 8 bytes
    pub end_date: i64,           // 8 bytes
    pub status: u8,              // 1 byte
    pub yes_votes: u64,          // 8 bytes
    pub no_votes: u64,           // 8 bytes
    pub total_voting_power: u64, // 8 bytes
    pub bump: u8,                // 1 byte
}

impl Proposal {
    pub const MAX_TITLE_LENGTH: usize = 128;
    /// Calculates the space needed for the account
    pub const fn space() -> usize {
        8 +                    // account discriminator
        32 +                   // artist Pubkey
        8 +                    // proposal_id
        4 + Self::MAX_TITLE_LENGTH + // title String (length prefix + max)
        8 +                    // number_of_tokens
        8 + 8 +                // start_date + end_date
        1 +                    // status
        8 + 8 + 8 +            // yes_votes + no_votes + total_voting_power
        1 // bump
    }
}
