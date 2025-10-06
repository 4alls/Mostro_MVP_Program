
use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
	pub artist: Pubkey,
	pub proposal_id: u64,
	pub title: String,
	pub number_of_tokens: u64,
	pub start_date: i64,
	pub end_date: i64,
	pub status: u8,
	pub yes_votes: u64,
	pub no_votes: u64,
	pub total_voting_power: u64,
	pub bump: u8,
}
