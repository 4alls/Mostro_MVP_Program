#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::state::Proposal;

#[derive(Accounts)]
#[instruction(name: String, proposal_id: u64)]
pub struct FinalizeProposal<'info> {
    /// Any user can call to finalize after the voting period
    #[account(mut)]
    pub caller: Signer<'info>,

    /// Proposal PDA to finalize
    #[account(
        mut,
        seeds = [
            b"artist_proposal",
            name.as_bytes().as_ref(),
            proposal_id.to_le_bytes().as_ref()
        ],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}

pub fn finalize_proposal_handler(
    ctx: Context<FinalizeProposal>,
    _name: String,
    _proposal_id: u64,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;

    // Ensure voting period has ended
    require!(
        clock.unix_timestamp >= proposal.end_date,
        ErrorCode::VotingStillActive
    );

    let total_votes = proposal.yes_votes + proposal.no_votes;

    // Quorum: at least 10% of total tokens must participate
    let quorum = proposal.number_of_tokens / 10; // 10%
    if total_votes < quorum {
        proposal.status = 2; // Rejected due to low participation
        return Ok(());
    }

    // Approval threshold: 51% yes votes
    if proposal.yes_votes * 100 / total_votes >= 51 {
        proposal.status = 1; // Approved
    } else {
        proposal.status = 2; // Rejected
    }

    Ok(())
}
