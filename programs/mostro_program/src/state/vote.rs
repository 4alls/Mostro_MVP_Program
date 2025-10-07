
use anchor_lang::prelude::*;

#[account]
pub struct Vote {
	pub proposal: Pubkey,
	pub voter: Pubkey,
	pub vote_choice: bool,
	pub voting_power: u64,
	pub bump: u8,
}
