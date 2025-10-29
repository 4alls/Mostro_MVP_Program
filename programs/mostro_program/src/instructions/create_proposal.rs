use anchor_lang::prelude::*; 
use crate::state::{Artist, Proposal};
use crate::instructions::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub artist: Account<'info, Artist>,

    #[account(
        init,
        payer = creator,
        space = Proposal::space(),
        seeds = [b"proposal", artist.key().as_ref(), creator.key().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}

pub fn create_proposal_handler(
    ctx: Context<CreateProposal>,
    title: String,
    description: String,
    number_of_tokens: u64,
    milestone_reached: bool,
    early_access: bool,
    bump: u8,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;

    // --- Link proposal to artist identity ---
    proposal.artist = ctx.accounts.artist.key();  // now valid
    proposal.creator = ctx.accounts.creator.key();

    // --- Proposal content ---
    proposal.title = title;
    proposal.description = description;

    // --- Voting / token allocation initialization ---
    proposal.number_of_tokens = number_of_tokens;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;

    // --- Set proposal duration (default: 10 days) ---
    proposal.start_date = clock.unix_timestamp;
    proposal.end_date = clock.unix_timestamp + 10 * 24 * 60 * 60;

    // --- Status: 0 = Active, 1 = Approved, 2 = Rejected ---
    proposal.status = 0;

    // --- Additional flags ---
    proposal.milestone_reached = milestone_reached;
    proposal.early_access = early_access;

    // --- Financial tracking ---
    proposal.usdc_collected = 0;
    proposal.artist_tokens_sold = 0;

    // --- Store bump for PDA validation ---
    proposal.bump = bump;

    Ok(())
}

