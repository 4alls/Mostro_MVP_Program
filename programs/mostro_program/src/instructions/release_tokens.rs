use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::Proposal;
use crate::constants::ARTIST_VAULT_SEED_PREFIX;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // Admin calls this function

    /// Proposal account
    #[account(
        mut,
        constraint = proposal.status == 1 @ ErrorCode::ProposalNotApproved // 1 = Approved
    )]
    pub proposal: Account<'info, Proposal>,

    /// Artist’s wallet receiving tokens
    /// CHECK: just storing the public key
    pub artist_wallet: UncheckedAccount<'info>,

    /// Program-owned vault holding tokens
    #[account(
        mut,
        seeds = [ARTIST_VAULT_SEED_PREFIX, artist_wallet.key().as_ref()],
        bump
    )]
    pub artist_vault: Account<'info, TokenAccount>,

    /// PDA authority that owns the artist vault
    /// CHECK: program signer derived from the same seeds
    #[account(
        seeds = [ARTIST_VAULT_SEED_PREFIX, artist_wallet.key().as_ref()],
        bump
    )]
    pub artist_vault_authority: UncheckedAccount<'info>,

    /// Token account of artist
    #[account(mut)]
    pub artist_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn release_tokens_to_artist_handler(ctx: Context<ReleaseTokens>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    // -----------------------------
    // Check that the proposal artist matches the provided wallet
    // -----------------------------
    require!(
        proposal.artist == ctx.accounts.artist_wallet.key(),
        ErrorCode::InvalidArtist
    );

    // -----------------------------
    // Check that the proposal is approved
    // -----------------------------
    require!(
        proposal.status == 1, // 1 = Approved
        ErrorCode::ProposalNotApproved
    );

    // -----------------------------
    // Amount to release
    // -----------------------------
    let amount = proposal.number_of_tokens;

    // -----------------------------
    // Prepare seeds for PDA authority signing
    // -----------------------------
    
    let bump = ctx.bumps.artist_vault_authority;
    let seeds = &[
        ARTIST_VAULT_SEED_PREFIX, // already &[u8]
        ctx.accounts.artist_wallet.key.as_ref(),
        &[bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // -----------------------------
    // Transfer tokens from vault to artist
    // -----------------------------
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.artist_vault.to_account_info(),
        to: ctx.accounts.artist_token_account.to_account_info(),
        authority: ctx.accounts.artist_vault_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds),
        amount,
    )?;

    // -----------------------------
    // Mark proposal as executed
    // -----------------------------
    proposal.status = 3; // 3 = Executed

    Ok(())
}
