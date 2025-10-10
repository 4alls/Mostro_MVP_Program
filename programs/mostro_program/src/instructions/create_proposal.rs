#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;
use crate::state::{Artist, Proposal};

#[derive(Accounts)]
#[instruction(name: String, proposal_id: u64, title: String)]
pub struct CreateProposal<'info> {
    /// Account paying for the creation of the proposal PDA
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    /// Artist PDA associated with the proposal
    #[account(
        mut,
        seeds = [b"artist", name.as_bytes().as_ref()],
        bump
    )]
    pub artist: Account<'info, Artist>,

    /// Proposal PDA, initialized for this new proposal
    #[account(
        init,
        payer = fee_payer,
        space = Proposal::space(), // <-- uses the one in state/proposal.rs
        seeds = [
            b"artist_proposal",
            name.as_bytes().as_ref(),
            proposal_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    /// Authority of the artist creating the proposal
    pub artist_authority: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}

/// Handler to initialize a proposal
pub fn create_proposal_handler(
    ctx: Context<CreateProposal>,
    _name: String, // only used for PDA derivation
    proposal_id: u64,
    title: String,
    number_of_tokens: u64,
) -> Result<()> {
    // Validate title length
    if title.len() > Proposal::MAX_TITLE_LENGTH {
        return Err(ProgramError::InvalidInstructionData.into());
    }

    let artist = &ctx.accounts.artist;

    // Ensure only artist authority can create the proposal
    require_keys_eq!(ctx.accounts.artist_authority.key(), artist.artist_wallet);

    let proposal = &mut ctx.accounts.proposal;

    // Initialize basic fields
    proposal.proposal_id = proposal_id;
    proposal.title = title;
    proposal.artist = artist.key();
    proposal.number_of_tokens = number_of_tokens;

    // Initialize default values
    proposal.start_date = Clock::get()?.unix_timestamp;
    proposal.end_date = proposal.start_date + 7 * 24 * 60 * 60; // 1 week
    proposal.status = 0; // e.g., 0 = pending
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.total_voting_power = 0;

    // Store bump
    proposal.bump = ctx.bumps.proposal;

    Ok(())
}
