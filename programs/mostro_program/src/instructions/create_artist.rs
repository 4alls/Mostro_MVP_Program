use anchor_lang::prelude::*;
use crate::state::Artist;

#[derive(Accounts)]
pub struct CreateArtist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Artist::space(),
        seeds = [b"artist", artist_name.as_bytes()],
        bump
    )]
    pub artist: Account<'info, Artist>,

    pub system_program: Program<'info, System>,
}

pub fn create_artist_handler(
    ctx: Context<CreateArtist>,
    artist_name: String,
    image: String,
    latest_single_title: String,
    latest_single_duration: String,
    mint: Pubkey,
    total_tokens: u64
) -> Result<()> {
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

