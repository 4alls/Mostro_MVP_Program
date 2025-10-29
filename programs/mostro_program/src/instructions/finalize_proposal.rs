use anchor_lang::prelude::*;
use crate::state::Proposal;
use crate::error::ErrorCode;

/// Accounts required to finalize a proposal
#[derive(Accounts)]
#[instruction(title: String)]
pub struct FinalizeProposal<'info> {
    #[account(mut)]
    pub caller: Signer<'info>, // Any wallet that calls the finalize instruction (could be admin or any user)

    /// The proposal account to finalize
    #[account(
        mut,
        seeds = [b"proposal", title.as_bytes()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}

/// Handler to finalize a proposal:
/// 1. Ensures voting period has ended or milestone reached.
/// 2. Checks quorum and determines proposal status.
/// 3. Sets proposal status to Approved (1) or Rejected (2)
pub fn finalize_proposal_handler(
    ctx: Context<FinalizeProposal>,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?; // Get current blockchain timestamp

    // --- Step 1: Ensure voting period is over or milestone reached ---
    require!(
        clock.unix_timestamp >= proposal.end_date || proposal.milestone_reached,
        ErrorCode::VotingStillActive // Prevent early finalization
    );

    // --- Step 2: Compute quorum and total votes ---
    let total_votes = proposal.yes_votes + proposal.no_votes;
    let quorum = proposal.number_of_tokens / 10; // 10% of total tokens must participate

    // --- Step 3: Reject proposals with insufficient participation ---
    if total_votes < quorum {
        proposal.status = 2; // Rejected due to low participation
        return Ok(()); // Early return
    }

    // --- Step 4: Determine final outcome based on majority ---
    // If yes votes ≥ 51%, proposal approved
    proposal.status = if proposal.yes_votes * 100 / total_votes >= 51 {
        1 // Approved
    } else {
        2 // Rejected
    };

    Ok(())
}
