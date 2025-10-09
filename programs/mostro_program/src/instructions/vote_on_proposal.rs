use anchor_lang::prelude::*;
use crate::state::{Proposal, Vote};
use crate::error::*;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction(name: String, proposal_id: u64)]
pub struct VoteOnProposal<'info> {
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
        init_if_needed,
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

    /// Voter's token account (used to calculate voting power)
    pub voter_token_account: Account<'info, TokenAccount>,

    /// Token mint representing voting power
    pub token_mint: Account<'info, Mint>,

    /// System program
    pub system_program: Program<'info, System>,
}

/// Instruction handler for voting on a proposal
pub fn vote_on_proposal_handler(
    ctx: Context<VoteOnProposal>,
    _name: String,
    _proposal_id: u64,
    vote_choice: bool,
) -> Result<()> {
    let vote_account = &mut ctx.accounts.vote;
    let proposal = &mut ctx.accounts.proposal;

    // Initialize vote account
    // Original TODOs (kept for clarity):
    // TODO: Calculate voting power based on voter's token balance
    // TODO: Update proposal's vote counts
    // if vote_choice { proposal.yes_votes += vote_account.voting_power; }
    // else { proposal.no_votes += vote_account.voting_power; }
    // proposal.total_voting_power += vote_account.voting_power;

    // Validate proposal is active
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    if now < proposal.start_date {
        return err!(crate::error::ErrorCode::ProposalNotActive);
    }
    if now > proposal.end_date {
        return err!(crate::error::ErrorCode::ProposalExpired);
    }

    // Prevent double voting: if the vote account already has a voter set, the user already voted
    if vote_account.voter != Pubkey::default() {
        return err!(crate::error::ErrorCode::AlreadyVoted);
    }

    // Calculate voting power based on voter's token account balance
    // Validate token mint matches expected mint
    let voter_token_account = &ctx.accounts.voter_token_account;
    if voter_token_account.mint != ctx.accounts.token_mint.key() {
        return err!(crate::error::ErrorCode::Unauthorized);
    }

    // Optional: ensure token account owner is the signer
    if voter_token_account.owner != ctx.accounts.voter.key() {
        return err!(crate::error::ErrorCode::Unauthorized);
    }

    let voting_power = voter_token_account.amount;
    if voting_power == 0 {
        return err!(crate::error::ErrorCode::NoVotingPower);
    }

    // Fill vote account
    vote_account.proposal = proposal.key();
    vote_account.voter = ctx.accounts.voter.key();
    vote_account.vote_choice = vote_choice;
    vote_account.voting_power = voting_power;
    vote_account.bump = ctx.bumps.vote;

    // Update proposal's vote counts
    if vote_choice {
        proposal.yes_votes = proposal.yes_votes.saturating_add(voting_power);
    } else {
        proposal.no_votes = proposal.no_votes.saturating_add(voting_power);
    }
    proposal.total_voting_power = proposal.total_voting_power.saturating_add(voting_power);

    Ok(())
}
