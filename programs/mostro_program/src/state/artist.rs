
use anchor_lang::prelude::*;

#[account]
pub struct Artist {
	pub name: String,
	pub description: String,
	pub token_mint: Pubkey,
	pub bonding_curve_vault: Pubkey,
	pub artist_vault: Pubkey,
	pub total_supply: u64,
	pub tokens_sold: u64,
	pub sol_raised: u64,
	pub proposal_count: u64,
	pub bump: u8,
}
