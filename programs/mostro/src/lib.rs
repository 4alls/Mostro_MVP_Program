
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use std::str::FromStr;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("2SYi3NFHTnCXHEzxNpa8nEyehkmZPyikbCarmxngSdTn");

#[program]
pub mod mostro {
    use super::*;

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
	pub fn create_config(ctx: Context<CreateConfig>, percentage_bonding_curve: u8, percentage_artist: u8, percentage_mostro: u8, number_of_sol_to_migrate: u64) -> Result<()> {
		create_config::handler(ctx, percentage_bonding_curve, percentage_artist, percentage_mostro, number_of_sol_to_migrate)
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
	pub fn create_artist(ctx: Context<CreateArtist>, name: String, description: String) -> Result<()> {
		create_artist::handler(ctx, name, description)
	}

/// Purchase tokens from bonding curve
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[]` config: [Config] Platform configuration
/// 2. `[writable]` artist: [Artist] Artist account
/// 3. `[writable, signer]` buyer: [AccountInfo] Token buyer
/// 4. `[writable, signer]` buyer_token_account: [AccountInfo] Buyer's token account
/// 5. `[]` bonding_curve_vault: [AccountInfo] Bonding curve vault authority
/// 6. `[writable]` bonding_curve_token_account: [AccountInfo] Bonding curve token account
/// 7. `[]` token_mint: [Mint] Token mint
/// 8. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 9. `[writable]` source: [AccountInfo] The source account.
/// 10. `[]` mint: [Mint] The token mint.
/// 11. `[writable]` destination: [AccountInfo] The destination account.
/// 12. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 13. `[]` token_program: [AccountInfo] Auto-generated, TokenProgram
///
/// Data:
/// - name: [String] Artist name
/// - sol_amount: [u64] SOL amount to spend in lamports
	pub fn buy_token(ctx: Context<BuyToken>, name: String, sol_amount: u64) -> Result<()> {
		buy_token::handler(ctx, name, sol_amount)
	}

/// Sell tokens back to bonding curve
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[]` config: [Config] Platform configuration
/// 2. `[writable]` artist: [Artist] Artist account
/// 3. `[writable, signer]` seller: [AccountInfo] Token seller
/// 4. `[writable]` seller_token_account: [AccountInfo] Seller's token account
/// 5. `[]` bonding_curve_vault: [AccountInfo] Bonding curve vault authority
/// 6. `[writable]` bonding_curve_token_account: [AccountInfo] Bonding curve token account
/// 7. `[]` token_mint: [Mint] Token mint
/// 8. `[writable]` source: [AccountInfo] The source account.
/// 9. `[]` mint: [Mint] The token mint.
/// 10. `[writable]` destination: [AccountInfo] The destination account.
/// 11. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 12. `[]` token_program: [AccountInfo] Auto-generated, TokenProgram
///
/// Data:
/// - name: [String] Artist name
/// - token_amount: [u64] Token amount to sell
	pub fn sell_token(ctx: Context<SellToken>, name: String, token_amount: u64) -> Result<()> {
		sell_token::handler(ctx, name, token_amount)
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
	pub fn create_proposal(ctx: Context<CreateProposal>, name: String, proposal_id: u64, title: String, number_of_tokens: u64) -> Result<()> {
		create_proposal::handler(ctx, name, proposal_id, title, number_of_tokens)
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
	pub fn vote_on_proposal(ctx: Context<VoteOnProposal>, name: String, proposal_id: u64, vote_choice: bool) -> Result<()> {
		vote_on_proposal::handler(ctx, name, proposal_id, vote_choice)
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
	pub fn execute_proposal(ctx: Context<ExecuteProposal>, name: String, proposal_id: u64) -> Result<()> {
		execute_proposal::handler(ctx, name, proposal_id)
	}



}
