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
)]
pub struct ExecuteProposal<'info> {
	#[account(
		mut,
	)]
	pub fee_payer: Signer<'info>,

	#[account(
		seeds = [
			b"config",
		],
		bump,
	)]
	pub config: Account<'info, Config>,

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
		mut,
	)]
	/// CHECK: implement manual checks if needed
	pub artist_authority: UncheckedAccount<'info>,

	#[account(
		mut,
	)]
	/// CHECK: implement manual checks if needed
	pub artist_token_account: UncheckedAccount<'info>,

	#[account(
		mut,
		seeds = [
			b"vault",
			name.as_bytes().as_ref(),
		],
		bump,
	)]
	/// CHECK: implement manual checks if needed
	pub bonding_curve_vault: UncheckedAccount<'info>,

	#[account(
		mut,
	)]
	/// CHECK: implement manual checks if needed
	pub bonding_curve_token_account: UncheckedAccount<'info>,

	pub token_mint: Account<'info, Mint>,

	#[account(
		mut,
	)]
	/// CHECK: implement manual checks if needed
	pub source: UncheckedAccount<'info>,

	pub mint: Account<'info, Mint>,

	#[account(
		mut,
	)]
	/// CHECK: implement manual checks if needed
	pub destination: UncheckedAccount<'info>,

	pub authority: Signer<'info>,

	pub token_program: Program<'info, Token>,
}

impl<'info> ExecuteProposal<'info> {
	pub fn cpi_token_transfer_checked(&self, amount: u64, decimals: u8) -> Result<()> {
		anchor_spl::token::transfer_checked(
			CpiContext::new(self.token_program.to_account_info(), 
				anchor_spl::token::TransferChecked {
					from: self.source.to_account_info(),
					mint: self.mint.to_account_info(),
					to: self.destination.to_account_info(),
					authority: self.authority.to_account_info()
				}
			),
			amount, 
			decimals, 
		)
	}
}


/// Execute approved proposal (sell tokens, send SOL to artist)
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[]` config: [Config] Platform configuration
/// 2. `[writable]` artist: [Artist] Artist account
/// 3. `[writable]` proposal: [Proposal] Proposal to execute
/// 4. `[writable]` artist_authority: [AccountInfo] Artist's wallet to receive SOL
/// 5. `[writable]` artist_token_account: [AccountInfo] Artist's token account
/// 6. `[writable]` bonding_curve_vault: [AccountInfo] Bonding curve vault authority
/// 7. `[writable]` bonding_curve_token_account: [AccountInfo] Bonding curve token account
/// 8. `[]` token_mint: [Mint] Token mint
/// 9. `[writable]` source: [AccountInfo] The source account.
/// 10. `[]` mint: [Mint] The token mint.
/// 11. `[writable]` destination: [AccountInfo] The destination account.
/// 12. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 13. `[]` token_program: [AccountInfo] Auto-generated, TokenProgram
///
/// Data:
/// - name: [String] Artist name
/// - proposal_id: [u64] Proposal ID
pub fn handler(
	ctx: Context<ExecuteProposal>,
	name: String,
	proposal_id: u64,
) -> Result<()> {
    // Implement your business logic here...
	
	// Cpi calls wrappers
	ctx.accounts.cpi_token_transfer_checked(
		Default::default(),
		Default::default(),
	)?;

	Ok(())
}
