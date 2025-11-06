use anchor_lang::prelude::*;

use crate::state::*;
// CHANGES HERE, WE WON'T NEED THE LINE AFTER IF ARTIST IS CORRECTLY IMPORTED IN STATE
use crate::state::artist::Artist;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreatePerk<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub artist: Account<'info, Artist>,

    #[account(
        init,
        payer = signer,
        space = 8 + Perk::INIT_SPACE,
        seeds = [
            b"perk",
            title.as_bytes().as_ref(),
            artist.key().as_ref()
        ],
        bump
    )]
    pub perk: Account<'info, Perk>,

    pub system_program: Program<'info, System>,
}

pub fn create_perk(
    ctx: Context<CreatePerk>,
    title: String,
    description: String,
    // NEED TO BE AUTOMATED WITH AN ORACLE IN THE FUTURE => CHAINLINK CLIENT
    price_in_usdc: u64,
    price_in_tokens: u64,
) -> Result<()> {

    let artist = &ctx.accounts.artist;
    let perk = &mut ctx.accounts.perk;
    
    perk.artist = artist.key();
    perk.title = title;
    perk.description = description;
    perk.price_in_usdc = price_in_usdc;
    perk.price_in_tokens = price_in_tokens;

    Ok(())
}


