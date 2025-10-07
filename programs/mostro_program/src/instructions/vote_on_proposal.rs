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
	vote_choice: bool,
)]
pub struct VoteOnProposal<'info> {
	#[account(
		mut,
	)]
	pub fee_payer: Signer<'info>,

	#[account(
		mut,
		seeds = [
			b"artist_proposal",
			name.as_bytes().as_ref(),
			proposal_id.to_le_bytes().as_ref(),
		],
		bump,
	)]
	pub proposal: Account<'info, Proposal>,

	#[account(
		init,
		space=82,
		payer=fee_payer,
		seeds = [
			b"vote",
			proposal.key().as_ref(),
			voter.key().as_ref(),
		],
		bump,
	)]
	pub vote: Account<'info, Vote>,

	pub voter: Signer<'info>,

	/// CHECK: implement manual checks if needed
	pub voter_token_account: UncheckedAccount<'info>,

	pub token_mint: Account<'info, Mint>,

	pub system_program: Program<'info, System>,
}

/// Token holders vote on proposals
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable]` proposal: [Proposal] Proposal to vote on
/// 2. `[writable]` vote: [Vote] Vote account to create
/// 3. `[signer]` voter: [AccountInfo] Voter's wallet
/// 4. `[]` voter_token_account: [AccountInfo] Voter's token account
/// 5. `[]` token_mint: [Mint] Token mint
/// 6. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - name: [String] Artist name
/// - proposal_id: [u64] Proposal ID
/// - vote_choice: [bool] Vote choice (true=yes, false=no)
pub fn handler(
	ctx: Context<VoteOnProposal>,
	name: String,
	proposal_id: u64,
	vote_choice: bool,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
