use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{Artist, Config};
use crate::state::MAX_URI_LEN;
use crate::error::ErrorCode;

pub const ARTIST_SEED_PREFIX: &[u8] = b"artist";
pub const ARTIST_VAULT_SEED_PREFIX: &[u8] = b"artist_vault";

#[derive(Accounts)]
pub struct CreateArtist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // Only admin can create artists

    /// CHECK: just storing the public key
    pub admin_account: UncheckedAccount<'info>, // Used as authority if needed

    /// The artist’s wallet (they don’t control the vault)
    /// CHECK: just storing the public key
    pub artist_wallet: UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        space = Artist::space(MAX_URI_LEN), // uses state::MAX_URI_LEN
        seeds = [ARTIST_SEED_PREFIX, artist_wallet.key().as_ref()],
        bump
    )]
    pub artist_account: Account<'info, Artist>,

    #[account(mut)]
    pub pump_token_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"config_mostro"],
        bump
    )]
    pub global_config: Account<'info, Config>,

    /// CHECK: program-derived authority for artist vault
    #[account(
        seeds = [ARTIST_VAULT_SEED_PREFIX, artist_wallet.key().as_ref()],
        bump
    )]
    pub artist_vault_authority: UncheckedAccount<'info>,

    /// Program-owned vault holding the artist’s tokens
    #[account(
        init,
        payer = admin,
        token::mint = pump_token_mint,
        token::authority = artist_vault_authority, // PDA itself is the owner
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
    // Ensure only admin can create an artist
    require_keys_eq!(
        ctx.accounts.admin.key(),
        ctx.accounts.global_config.admin_wallet,
        ErrorCode::UnauthorizedAdmin
    );

    let artist_account = &mut ctx.accounts.artist_account;
    let config = &ctx.accounts.global_config;

    // Initialize artist account fields
    artist_account.artist_wallet = ctx.accounts.artist_wallet.key();
    artist_account.pump_token_mint = ctx.accounts.pump_token_mint.key();
    artist_account.metadata_uri = metadata_uri;
    artist_account.artist_vault = ctx.accounts.artist_vault.key();
    artist_account.percentage_artist = percentage_artist.unwrap_or(0);
    artist_account.percentage_mostro = percentage_mostro.unwrap_or(0);
    artist_account.global_config = config.key();
    artist_account.bump = ctx.bumps.artist_account;

    Ok(())
}
