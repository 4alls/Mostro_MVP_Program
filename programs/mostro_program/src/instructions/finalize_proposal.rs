use anchor_lang::prelude::*;
use crate::state::Proposal;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct FinalizeProposal<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,

    #[account(
        mut,
        seeds = [b"proposal", title.as_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}

pub fn finalize_proposal_handler(
    ctx: Context<FinalizeProposal>,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;

    require!(
        clock.unix_timestamp >= proposal.end_date || proposal.milestone_reached,
        ErrorCode::VotingStillActive
    );

    let total_votes = proposal.yes_votes + proposal.no_votes;
    let quorum = proposal.number_of_tokens / 10; // 10% threshold

    if total_votes < quorum {
        proposal.status = 2; // Rejected due to low participation
        return Ok(());
    }

    proposal.status = if proposal.yes_votes * 100 / total_votes >= 51 { 1 } else { 2 };
    Ok(())
}
