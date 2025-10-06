
use anchor_lang::prelude::*;

#[account] // tells Anchor this is an on-chain account with serializable data.
pub struct Config {
	pub percentage_bonding_curve: u8,
	pub percentage_artist: u8,
	pub percentage_mostro: u8,
	pub admin: Pubkey, // Public key of the program admin
	pub number_of_sol_to_migrate: u64, // Some protocol-wide counter/value.
	pub bump: u8, // Stores the PDA bump so the program can sign as the PDA later
}
