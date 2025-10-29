use anchor_lang::prelude::*;
use crate::state::{Proposal, Vote};
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct VoteProposal<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        mut,
        seeds = [b"proposal", title.as_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}

pub fn vote_proposal_handler(
    ctx: Context<VoteProposal>,
    vote_yes: bool,
    voter_token_balance: u64
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;

    require!(clock.unix_timestamp <= proposal.end_date, ErrorCode::VotingStillActive);

    if vote_yes {
        proposal.yes_votes += voter_token_balance;
    } else {
        proposal.no_votes += voter_token_balance;
    }

    // Optional: store Vote PDA if needed
    // let vote = &mut ctx.accounts.vote;
    // vote.voter = ctx.accounts.voter.key();
    // vote.proposal = proposal.key();
    // vote.vote_yes = vote_yes;
    // vote.token_weight = voter_token_balance;

    Ok(())
}
