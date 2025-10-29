use anchor_lang::prelude::*;
use crate::state::Proposal;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub creator: Signer<'info>, // Artist creating the proposal

    #[account(
        init,
        payer = creator,
        space = Proposal::space(),
        seeds = [b"proposal", title.as_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}

pub fn create_proposal_handler(
    ctx: Context<CreateProposal>,
    title: String,
    description: String,
    number_of_tokens: u64,
    milestone_reached: bool,
    early_access: bool,
    bump: u8
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let clock = Clock::get()?;

    proposal.title = title;
    proposal.description = description;
    proposal.creator = ctx.accounts.creator.key();
    proposal.number_of_tokens = number_of_tokens;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.start_date = clock.unix_timestamp;
    proposal.end_date = clock.unix_timestamp + 10 * 24 * 60 * 60; // 10 days
    proposal.status = 0; // Active
    proposal.milestone_reached = milestone_reached;
    proposal.usdc_collected = 0;
    proposal.artist_tokens_sold = 0;
    proposal.bump = bump;
    proposal.early_access = early_access;

    Ok(())
}
