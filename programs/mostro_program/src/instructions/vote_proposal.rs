#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;
use crate::state::{Proposal, Vote};
use crate::error::ErrorCode;
use anchor_spl::token::{Mint, TokenAccount};

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

    /// Voter's SPL token account used to determine voting power
    pub voter_token_account: Account<'info, TokenAccount>,

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
    let proposal = &mut ctx.accounts.proposal;

    // Check voting period 
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp <= proposal.end_date,
        ErrorCode::VotingEnded
    );

     // Check if user has already voted
    require!(
        ctx.accounts.vote.to_account_info().data_is_empty(),
        ErrorCode::AlreadyVoted
    );

    let vote_account = &mut ctx.accounts.vote;

    // Initialize vote account
    vote_account.proposal = proposal.key();
    vote_account.voter = ctx.accounts.voter.key();
    vote_account.vote_choice = vote_choice;

    // Calculate voting power based on voter's token balance
    let voter_token_account: &Account<TokenAccount> = &ctx.accounts.voter_token_account;
    let voting_power = voter_token_account.amount;
    require!(voting_power > 0, ErrorCode::NoVotingPower);
    
    vote_account.voting_power = voting_power;
    vote_account.bump = ctx.bumps.vote;

    // Update proposal tallies
    if vote_choice {
        proposal.yes_votes = proposal
            .yes_votes
            .checked_add(vote_account.voting_power)
            .ok_or(ErrorCode::Overflow)?;
    } else {
        proposal.no_votes = proposal
        .no_votes
        .checked_add(vote_account.voting_power)
        .ok_or(ErrorCode::Overflow)?;
    }

    // Track total votes for approval threshold
    proposal.total_voting_power = proposal
        .total_voting_power
        .checked_add(vote_account.voting_power)
        .ok_or(ErrorCode::Overflow)?;

    Ok(())
}
