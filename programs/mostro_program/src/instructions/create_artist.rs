#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{Artist, Config};
use crate::error::ErrorCode;

pub const ARTIST_SEED_PREFIX: &[u8] = b"artist";
pub const ARTIST_VAULT_SEED_PREFIX: &[u8] = b"artist_vault";

#[derive(Accounts)]
pub struct CreateArtist<'info> {
    /// Admin creating the artist
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: just storing public key, not used for security checks
    pub admin_account: UncheckedAccount<'info>,

    /// The artist’s personal wallet (not the vault)
    /// CHECK: just storing public key
    pub artist_wallet: UncheckedAccount<'info>,

    /// PDA storing artist data
    #[account(
        init,
        payer = admin,
        space = Artist::space(),
        seeds = [ARTIST_SEED_PREFIX, artist_wallet.key().as_ref()],
        bump
    )]
    pub artist_account: Account<'info, Artist>,

    /// The token mint associated with the artist’s vault
    #[account(mut)]
    pub pump_token_mint: Account<'info, Mint>,

    /// Global configuration account (singleton)
    #[account(
        seeds = [b"global_config"],
        bump
    )]
    pub global_config: Account<'info, Config>,

    /// CHECK: PDA authority controlling the artist’s vault
    #[account(
        seeds = [ARTIST_VAULT_SEED_PREFIX, artist_wallet.key().as_ref()],
        bump
    )]
    pub artist_vault_authority: UncheckedAccount<'info>,

    /// Program-owned vault that holds tokens for this artist
    #[account(
        init,
        payer = admin,
        token::mint = pump_token_mint,
        token::authority = artist_vault_authority,
        seeds = [ARTIST_VAULT_SEED_PREFIX, artist_wallet.key().as_ref()],
        bump
    )]
    pub artist_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_artist_handler(
    ctx: Context<CreateArtist>,
    metadata_uri: String,
    percentage_artist: Option<u8>,
    percentage_mostro: Option<u8>,
) -> Result<()> {
    // Only admin can create artists
    require_keys_eq!(
        ctx.accounts.admin.key(),
        ctx.accounts.global_config.admin_wallet,
        ErrorCode::UnauthorizedAdmin
    );

    // Resolve percentages, defaulting to global config values
    let pct_artist = percentage_artist.unwrap_or(ctx.accounts.global_config.percentage_artist);
    let pct_mostro = percentage_mostro.unwrap_or(ctx.accounts.global_config.percentage_mostro);

    // Validate percentages
    require!(
        pct_artist + pct_mostro <= 100,
        ErrorCode::InvalidPercentage
    );

    // Initialize artist account
    let artist = &mut ctx.accounts.artist_account;

    artist.artist_wallet = ctx.accounts.artist_wallet.key();
    artist.pump_token_mint = ctx.accounts.pump_token_mint.key();
    artist.metadata_uri = metadata_uri;
    artist.artist_vault = ctx.accounts.artist_vault.key();
    artist.percentage_artist = pct_artist;
    artist.percentage_mostro = pct_mostro;
    artist.global_config = ctx.accounts.global_config.key();
    artist.bump = ctx.bumps.artist_account; 

    Ok(())
}
