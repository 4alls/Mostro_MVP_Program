use anchor_lang::prelude::*;
use crate::state::Vote;

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub creator: Pubkey,
    pub number_of_tokens: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub start_date: i64,
    pub end_date: i64,
    pub status: u8,               // 0: Active, 1: Approved, 2: Rejected
    pub milestone_reached: bool,
    pub usdc_collected: u64,
    pub artist_tokens_sold: u64,
    pub bump: u8,                 // store bump explicitly
    pub early_access: bool,       // optional: distinguish campaign tokens
}

impl Proposal {
    pub fn space() -> usize {
        8 +               // discriminator
        4 + 100 +         // title
        4 + 500 +         // description
        32 +              // creator
        8 + 8 + 8 + 8 +   // number_of_tokens, yes_votes, no_votes, usdc_collected
        8 + 8 +           // start_date + end_date
        1 +               // status
        1 +               // milestone_reached
        8 +               // artist_tokens_sold
        1 +               // bump
        1                 // early_access
    }
}
