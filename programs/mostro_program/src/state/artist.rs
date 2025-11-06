use anchor_lang::prelude::*;

#[account]
pub struct Artist {
    pub name: String,
    pub image: String,
    pub latest_single: LatestSingle,
    pub total_tokens: u64,         // optional: total artist tokens
    pub campaign_tokens_sold: u64, // optional: track campaign sales
    pub mint: Pubkey,              // optional: SPL token mint
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LatestSingle {
    pub title: String,
    pub duration: String,
}

impl Artist {
    pub fn space() -> usize {
        8 +                        // discriminator
        4 + 50 +                    // name (max 50 chars)
        4 + 200 +                   // image URL (max 200 chars)
        4 + 50 + 4 + 10 +           // latest_single title & duration (max sizes)
        8 + 8 +                      // total_tokens + campaign_tokens_sold
        32 // mint
    }
}

/// USE InitSpace TO AUTOMATICALLY CALCULATE THE RIGHT SPACE FOR THE ACCOUNT
/// USE [max_len] TO SET MAX LENGTHS FOR VARIABLE-LENGTH FIELDS
/// REPLACE LatestSingle BY A VECTOR: Vec<LatestSingle>
/*

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Artist {
    #[max_len(50)]
    pub name: String,
    #[max_len(200)]
    pub image: String,
    pub latest_single: Vec<LatestSingle>,
    pub total_tokens: u64,         
    pub campaign_tokens_sold: u64, 
    pub mint: Pubkey,              
}
 */