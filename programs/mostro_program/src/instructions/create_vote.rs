use anchor_lang::prelude::*;
use crate::state::{Vote, Proposal};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        init,
        payer = voter,
        space = Vote::space(),
        seeds = [b"vote", proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote: Account<'info, Vote>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}

pub fn create_vote_handler(
    ctx: Context<CreateVote>,
    approve: bool, // true = yes, false = no
) -> Result<()> {
    let vote = &mut ctx.accounts.vote;
    let proposal = &mut ctx.accounts.proposal;

    // Ensure proposal is still active
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp < proposal.end_date && proposal.status == 0,
        ErrorCode::VotingEnded
    );

    vote.voter = ctx.accounts.voter.key();
    vote.proposal = proposal.key();
    vote.approve = approve;

    // Count vote immediately on-chain
    if approve {
        proposal.yes_votes += 1;
    } else {
        proposal.no_votes += 1;
    }

    // Check for milestone early approval
    let total_votes = proposal.yes_votes + proposal.no_votes;
    if proposal.milestone_reached && proposal.yes_votes > proposal.number_of_tokens / 2 {
        proposal.status = 1; // Approved
        proposal.end_date = clock.unix_timestamp; // Finalize early
    }

    Ok(())
}
