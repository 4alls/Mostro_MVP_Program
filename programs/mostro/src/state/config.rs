
use anchor_lang::prelude::*;

#[account]
pub struct Config {
	pub percentage_bonding_curve: u8,
	pub percentage_artist: u8,
	pub percentage_mostro: u8,
	pub admin: Pubkey,
	pub number_of_sol_to_migrate: u64,
	pub bump: u8,
}
