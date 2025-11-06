use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Perk {
    pub artist: Pubkey,   
    #[max_len(100)]
    pub title: String,
    #[max_len(500)]
    pub description: String,
    pub price_in_usdc: u64,
    pub price_in_tokens: u64,
}