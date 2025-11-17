use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug, PartialEq)]
pub struct Proposal {
    // --- Link to the artist ---
    pub artist: Pubkey,  // The artist account this proposal belongs to
    pub creator: Pubkey, // Wallet that created the proposal

    // --- Proposal content ---
    pub title: String,
    pub description: String,

    // --- Voting / token allocation ---
    pub number_of_tokens: u64,
    pub yes_votes: u64,
    pub no_votes: u64,

    // --- Timing ---
    pub start_date: i64,
    pub end_date: i64,

    // --- Status flags ---
    pub status: u8, // 0 = Active, 1 = Approved, 2 = Rejected
    pub milestone_reached: bool,
    pub early_access: bool,

    // --- Financial tracking ---
    pub usdc_collected: u64,
    pub artist_tokens_sold: u64,

    // --- PDA bump for this proposal ---
    pub bump: u8,
}

impl Proposal {
    pub fn space() -> usize {
        8 +               // Anchor account discriminator
        32 +              // artist
        32 +              // creator
        4 + 100 +         // title (max 100 bytes)
        4 + 500 +         // description (max 500 bytes)
        8 + 8 + 8 +       // number_of_tokens, yes_votes, no_votes
        8 + 8 +           // start_date, end_date
        1 +               // status
        1 + 1 +           // milestone_reached, early_access
        8 + 8 +           // usdc_collected, artist_tokens_sold
        1 // bump
    }
}

// USE InitSpace TO AUTOMATICALLY CALCULATE THE RIGHT SPACE FOR THE ACCOUNT
// USE [max_len] TO SET MAX LENGTHS FOR VARIABLE-LENGTH FIELDS

/*
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub artist: Pubkey,  
    pub creator: Pubkey, 
    #[max_len(100)]
    pub title: String,
    #[max_len(500)]
    pub description: String,
    pub number_of_tokens: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub start_date: i64,
    pub end_date: i64,
    pub status: u8, 
    pub milestone_reached: bool,
    pub early_access: bool,
    pub usdc_collected: u64,
    pub artist_tokens_sold: u64,
    pub bump: u8,
}
*/
