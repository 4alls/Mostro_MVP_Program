use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(
	percentage_bonding_curve: u8,
	percentage_artist: u8,
	percentage_mostro: u8,
	number_of_sol_to_migrate: u64,
)]
pub struct CreateConfig<'info> {
	#[account(
		mut,
	)]
	pub fee_payer: Signer<'info>, // the wallet that pays the rent to initialize the config account

	#[account(
		init,
		space=52,
		payer=fee_payer,
		seeds = [
			b"config",
		],
		bump,
	)]
	pub config: Account<'info, Config>, // the PDA account that will hold your global config.

	pub admin: Signer<'info>, // signer who will be set as the admin in the config

	pub system_program: Program<'info, System>, // required for creating new accounts on Solana
}

/// Initialize platform configuration
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable]` config: [Config] Platform configuration account
/// 2. `[signer]` admin: [AccountInfo] Admin wallet
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - percentage_bonding_curve: [u8] Percentage for bonding curve (default 87)
/// - percentage_artist: [u8] Percentage for artist (default 10)
/// - percentage_mostro: [u8] Percentage for Mostro (default 3)
/// - number_of_sol_to_migrate: [u64] SOL threshold for migration
pub fn handler(
	ctx: Context<CreateConfig>,
	percentage_bonding_curve: u8,
	percentage_artist: u8,
	percentage_mostro: u8,
	number_of_sol_to_migrate: u64,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
