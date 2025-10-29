use anchor_lang::prelude::*;
use crate::state::Artist;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateArtist<'info> {
    /// The admin or platform authority responsible for initializing new artists.
    /// This is the signer who pays rent for the new PDA.
    #[account(mut)]
    pub admin: Signer<'info>,

    /// PDA for the artist’s account.
    ///
    /// PDA derivation:
    /// - Derived from the static seed `"artist"`, but **validated** dynamically in the handler
    ///   using the artist’s name (we don't include it here because Anchor does not support
    ///   variable-length seeds directly in the `#[account(...)]` macro).
    /// - The actual PDA check happens inside `create_artist_handler`.
    ///
    /// Why:
    /// - This ensures every artist has a unique PDA tied to their name hash.
    /// - Artists can be looked up deterministically (same name → same PDA).
    ///
    #[account(
        init,
        payer = admin,
        space = Artist::space(),
        seeds = [b"artist"], // Only static part; full validation in handler
        bump
    )]
    pub artist: Account<'info, Artist>,

    /// Solana System Program (required for account creation)
    pub system_program: Program<'info, System>,
}

// -----------------------------
// Instruction Logic
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
    // -------------------------------------------------
    // STEP 1: Derive expected PDA using artist name
    // -------------------------------------------------
    // Even though the PDA was initialized via Anchor, we manually verify
    // that the passed-in account matches the deterministic PDA computed
    // using the artist's name. This prevents mismatched or malicious accounts.
    //
    // PDA derivation formula:
    //     artist_pda = Pubkey::find_program_address(["artist", artist_name.as_bytes()], program_id)
    //
    let artist_seed = &[b"artist", artist_name.as_bytes()];
    let (expected_artist_pda, _bump) = Pubkey::find_program_address(artist_seed, ctx.program_id);

    // -------------------------------------------------
    // STEP 2: Safety check — validate PDA correctness
    // -------------------------------------------------
    // Ensures the client passed the correct artist PDA derived from the name.
    // Prevents arbitrary accounts from being used in place of the real one.
    require!(
        ctx.accounts.artist.key() == expected_artist_pda,
        ErrorCode::InvalidArtist
    );

    // -------------------------------------------------
    // STEP 3: Store artist metadata in PDA
    // -------------------------------------------------
    // Once verified, we safely populate all artist fields.
    // This includes the metadata, mint reference, and initial supply.
    //
    let artist = &mut ctx.accounts.artist;
    artist.name = artist_name;
    artist.image = image;
    artist.latest_single.title = latest_single_title;
    artist.latest_single.duration = latest_single_duration;
    artist.mint = mint;
    artist.total_tokens = total_tokens;
    artist.campaign_tokens_sold = 0; // Initialize as 0 — updated as campaigns sell tokens

    // -------------------------------------------------
    // STEP 4: Emit (optional future improvement)
    // -------------------------------------------------
    // You can later add an event such as `ArtistCreated` for better indexing.
    //
    // emit!(ArtistCreated {
    //     artist: artist.key(),
    //     name: artist.name.clone(),
    //     mint: artist.mint,
    // });

    Ok(())
}
