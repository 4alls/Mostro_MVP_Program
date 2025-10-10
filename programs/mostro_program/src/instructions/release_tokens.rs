#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::error::ErrorCode;
use crate::instructions::create_artist::ARTIST_VAULT_SEED_PREFIX;

#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // Admin calls this function

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

/// Admin releases tokens to artist via program-controlled vault
pub fn release_tokens_to_artist_handler(
    ctx: Context<ReleaseTokens>,
    amount: u64,
) -> Result<()> {
    // Ensure only admin can release tokens
    require_keys_eq!(
        ctx.accounts.admin.key(),
        ctx.accounts.artist_vault_authority.key(), // or global_config.admin_wallet if you have it
        ErrorCode::UnauthorizedAdmin
    );

    // PDA bump and artist pubkey
    let vault_bump = ctx.bumps.artist_vault_authority;
    let artist_pubkey = ctx.accounts.artist_wallet.key();

    // Seeds for PDA signer
    let seeds: &[&[u8]] = &[
        ARTIST_VAULT_SEED_PREFIX,
        artist_pubkey.as_ref(),
        &[vault_bump],
    ];

    // Wrap in an extra reference for CpiContext::new_with_signer
    let signer_seeds: &[&[&[u8]]] = &[seeds];

    // Set up CPI transfer
    let cpi_accounts = Transfer {
        from: ctx.accounts.artist_vault.to_account_info(),
        to: ctx.accounts.artist_token_account.to_account_info(),
        authority: ctx.accounts.artist_vault_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

    // Execute the transfer
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
