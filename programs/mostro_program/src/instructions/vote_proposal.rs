#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;
use crate::state::{Proposal, Vote};
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(name: String, proposal_id: u64)]
pub struct VoteProposal<'info> {
    /// Account paying for transaction
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    /// Proposal PDA
    #[account(
        mut,
        seeds = [
            b"artist_proposal",
            name.as_bytes().as_ref(),
            proposal_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    /// Vote account for this voter
    #[account(
        init,
        payer = fee_payer,
        space = Vote::space(),
        seeds = [
            b"vote",
            proposal.key().as_ref(),
            voter.key().as_ref()
        ],
        bump
    )]
    pub vote: Account<'info, Vote>,

    /// The voter
    #[account(mut)]
    pub voter: Signer<'info>,

    /// CHECK: voter's token account (used to calculate voting power)
    pub voter_token_account: UncheckedAccount<'info>,

    /// Token mint representing voting power
    pub token_mint: Account<'info, Mint>,

    /// System program
    pub system_program: Program<'info, System>,
}

/// Instruction handler for voting on a proposal
pub fn vote_proposal_handler(
    ctx: Context<VoteProposal>,
    _name: String,
    _proposal_id: u64,
    vote_choice: bool,
) -> Result<()> {
    let vote_account = &mut ctx.accounts.vote;
    let proposal = &mut ctx.accounts.proposal;

    // Initialize vote account
    vote_account.proposal = proposal.key();
    vote_account.voter = ctx.accounts.voter.key();
    vote_account.vote_choice = vote_choice;

    // TODO: Calculate voting power based on voter's token balance
    vote_account.voting_power = 0;

    vote_account.bump = ctx.bumps.vote;

    // TODO: Update proposal's vote counts
    // if vote_choice { proposal.yes_votes += vote_account.voting_power; }
    // else { proposal.no_votes += vote_account.voting_power; }
    // proposal.total_voting_power += vote_account.voting_power;

    Ok(())
}
