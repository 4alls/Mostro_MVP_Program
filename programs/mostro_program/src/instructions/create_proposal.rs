use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(
	name: String,
	proposal_id: u64,
	title: String,
	number_of_tokens: u64,
)]
pub struct CreateProposal<'info> {
	#[account(
		mut,
	)]
	pub fee_payer: Signer<'info>,

	#[account(
		mut,
		seeds = [
			b"artist",
			name.as_bytes().as_ref(),
		],
		bump,
	)]
	pub artist: Account<'info, Artist>,

	#[account(
		init,
		space=202,
		payer=fee_payer,
		seeds = [
			b"artist_proposal",
			name.as_bytes().as_ref(),
			proposal_id.to_le_bytes().as_ref(),
		],
		bump,
	)]
	pub proposal: Account<'info, Proposal>,

	pub artist_authority: Signer<'info>,

	pub token_mint: Account<'info, Mint>,

	pub system_program: Program<'info, System>,
}

/// Artist creates governance proposal
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable]` artist: [Artist] Artist account
/// 2. `[writable]` proposal: [Proposal] Proposal account to create
/// 3. `[signer]` artist_authority: [AccountInfo] Artist's wallet authority
/// 4. `[]` token_mint: [Mint] Token mint for voting power calculation
/// 5. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - name: [String] Artist name
/// - proposal_id: [u64] Proposal ID
/// - title: [String] Proposal title
/// - number_of_tokens: [u64] Tokens to sell if approved
pub fn handler(
	ctx: Context<CreateProposal>,
	name: String,
	proposal_id: u64,
	title: String,
	number_of_tokens: u64,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
