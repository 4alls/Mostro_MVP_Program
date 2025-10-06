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
	description: String,
)]
pub struct CreateArtist<'info> {
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
		init,
		space=377,
		payer=fee_payer,
		seeds = [
			b"artist",
			name.as_bytes().as_ref(),
		],
		bump,
	)]
	pub artist: Account<'info, Artist>,

	pub artist_authority: Signer<'info>,

	#[account(
		init,
		payer = fee_payer,
		mint::decimals = 0,
	)]
	pub token_mint: Account<'info, Mint>,

	#[account(
		seeds = [
			b"vault",
			name.as_bytes().as_ref(),
		],
		bump,
	)]
	/// CHECK: implement manual checks if needed
	pub bonding_curve_vault: UncheckedAccount<'info>,

	#[account(
		signer,
		init,
		space=8,
		payer=fee_payer,
	)]
	/// CHECK: implement manual checks if needed
	pub bonding_curve_token_account: UncheckedAccount<'info>,

	#[account(
		signer,
		init,
		space=8,
		payer=fee_payer,
	)]
	/// CHECK: implement manual checks if needed
	pub artist_token_account: UncheckedAccount<'info>,

	#[account(
		signer,
		init,
		space=8,
		payer=fee_payer,
	)]
	/// CHECK: implement manual checks if needed
	pub mostro_token_account: UncheckedAccount<'info>,

	pub system_program: Program<'info, System>,

	#[account(mut)]
	pub mint: Account<'info, Mint>,

	#[account(
		mut,
		owner=Pubkey::from_str("11111111111111111111111111111111").unwrap(),
	)]
	pub funding: Signer<'info>,

	#[account(
		init,
		payer = funding,
		associated_token::mint = mint,
		associated_token::authority = wallet,
		associated_token::token_program = token_program,
	)]
	pub assoc_token_account: Account<'info, TokenAccount>,

	/// CHECK: implement manual checks if needed
	pub wallet: UncheckedAccount<'info>,

	pub token_program: Program<'info, Token>,

	pub owner: Signer<'info>,

	pub token_program: Program<'info, Token>,

	pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CreateArtist<'info> {
	pub fn cpi_token_initialize_mint2(&self, decimals: u8, mint_authority: Pubkey, freeze_authority: Option<Pubkey>) -> Result<()> {
		anchor_spl::token::initialize_mint2(
			CpiContext::new(self.token_program.to_account_info(), 
				anchor_spl::token::InitializeMint2 {
					mint: self.mint.to_account_info()
				}
			),
			decimals, 
			mint_authority, 
			freeze_authority, 
		)
	}
	pub fn cpi_token_mint_to(&self, amount: u64) -> Result<()> {
		anchor_spl::token::mint_to(
			CpiContext::new(self.token_program.to_account_info(), 
				anchor_spl::token::MintTo {
					mint: self.mint.to_account_info(),
					to: self.assoc_token_account.to_account_info(),
					authority: self.owner.to_account_info()
				}
			),
			amount, 
		)
	}
}


/// Create artist profile and Token2022 mint with distribution
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[]` config: [Config] Platform configuration
/// 2. `[writable]` artist: [Artist] Artist account to create
/// 3. `[signer]` artist_authority: [AccountInfo] Artist's wallet authority
/// 4. `[writable, signer]` token_mint: [Mint] Token2022 mint to create
/// 5. `[]` bonding_curve_vault: [AccountInfo] Vault authority for bonding curve tokens
/// 6. `[writable, signer]` bonding_curve_token_account: [AccountInfo] Token account for bonding curve vault
/// 7. `[writable, signer]` artist_token_account: [AccountInfo] Token account for artist vault
/// 8. `[writable, signer]` mostro_token_account: [AccountInfo] Token account for Mostro platform
/// 9. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 10. `[writable]` mint: [Mint] 
/// 11. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 12. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 13. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 14. `[]` token_program: [AccountInfo] SPL Token program
/// 15. `[signer]` owner: [AccountInfo] The mint's minting authority.
/// 16. `[]` token_program: [AccountInfo] Auto-generated, TokenProgram
/// 17. `[]` associated_token_program: [AccountInfo] Auto-generated, AssociatedTokenProgram
///
/// Data:
/// - name: [String] Artist name
/// - description: [String] type
pub fn handler(
	ctx: Context<CreateArtist>,
	name: String,
	description: String,
) -> Result<()> {
    // Implement your business logic here...
	
	// Cpi calls wrappers
	ctx.accounts.cpi_token_initialize_mint2(
		Default::default(),
	Pubkey::default(),
	None,
	)?;

	ctx.accounts.cpi_token_mint_to(
		Default::default(),
	)?;

	Ok(())
}
