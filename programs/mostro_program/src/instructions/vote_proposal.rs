use anchor_lang::prelude::*;
use crate::state::Proposal;
use crate::error::ErrorCode;

/// -----------------------------
/// Accounts for voting on a proposal
/// -----------------------------
#[derive(Accounts)]
#[instruction(proposal_title: String)]
pub struct VoteProposal<'info> {
    /// The wallet casting the vote
    #[account(mut)]
    pub voter: Signer<'info>,

    /// The proposal being voted on
    #[account(
        mut,
        seeds = [b"proposal", proposal_title.as_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,

    /// A minimal PDA to prevent double voting
    /// CHECK: Only existence matters; no data is read or written
    #[account(
        init,
        payer = voter,
        space = 8, // Only the 8-byte Anchor discriminator
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_marker: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// -----------------------------
/// Handler for voting
/// -----------------------------
pub fn vote_proposal_handler(
    ctx: Context<VoteProposal>,
    vote_yes: bool,           // true = yes, false = no
    voter_token_balance: u64, // number of tokens voter has for weighting
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;

    // --- Step 1: Ensure voting is still active ---
    require!(
        clock.unix_timestamp <= proposal.end_date && proposal.status == 0,
        ErrorCode::VotingEnded
    );

    // --- Step 2: Increment proposal tallies immediately ---
    if vote_yes {
        proposal.yes_votes += voter_token_balance;
    } else {
        proposal.no_votes += voter_token_balance; 
    }

    // --- Step 3: Optional early finalize if milestone & majority reached ---
    if proposal.milestone_reached && proposal.yes_votes > proposal.number_of_tokens / 2 {
        proposal.status = 1; // Approved
        proposal.end_date = clock.unix_timestamp; // Finalize early
    }

    // Note: existence of `vote_marker` PDA ensures voter cannot vote again
    Ok(())
}
