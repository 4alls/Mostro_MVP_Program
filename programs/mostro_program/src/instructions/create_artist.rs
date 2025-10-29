use anchor_lang::prelude::*;
use crate::state::Artist;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateArtist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The artist PDA account; initialized dynamically in the handler
    #[account(
        init,
        payer = admin,
        space = Artist::space(),
        seeds = [b"artist"], // only static part; name-based seed checked in handler
        bump
    )]
    pub artist: Account<'info, Artist>,

    pub system_program: Program<'info, System>,
}

// -----------------------------
// Handler
// -----------------------------
pub fn create_artist_handler(
    ctx: Context<CreateArtist>,
    artist_name: String,
    image: String,
    latest_single_title: String,
    latest_single_duration: String,
    mint: Pubkey,
    total_tokens: u64,
) -> Result<()> {
    // --- Step 1: Compute expected PDA using the artist name ---
    let artist_seed = &[b"artist", artist_name.as_bytes()];
    let (expected_artist_pda, _bump) = Pubkey::find_program_address(artist_seed, ctx.program_id);

    // --- Step 2: Validate that the passed-in account matches PDA (immutable borrow) ---
    require!(
        ctx.accounts.artist.key() == expected_artist_pda,
        ErrorCode::InvalidArtist
    );

    // --- Step 3: Now mutably borrow the artist account to write data ---
    let artist = &mut ctx.accounts.artist;

    artist.name = artist_name;
    artist.image = image;
    artist.latest_single.title = latest_single_title;
    artist.latest_single.duration = latest_single_duration;
    artist.mint = mint;
    artist.total_tokens = total_tokens;
    artist.campaign_tokens_sold = 0;

    Ok(())
}
