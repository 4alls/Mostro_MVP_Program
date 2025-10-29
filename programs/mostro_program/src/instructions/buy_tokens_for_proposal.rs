use anchor_lang::prelude::*;
use crate::state::{Proposal, VoterToken};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct BuyTokensForProposal<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    // PDA that will hold tokens & USDC
    #[account(mut)]
    pub proposal_vault: Account<'info, ProposalVault>,

    pub system_program: Program<'info, System>,
}

pub fn buy_tokens_for_proposal_handler(
    ctx: Context<BuyTokensForProposal>,
    amount_usdc: u64,
    artist_tokens_price: u64,
    is_campaign_purchase: bool, // determines early access
) -> Result<u64> {
    let proposal = &mut ctx.accounts.proposal;

    // Calculate how many artist tokens the user receives
    let tokens_bought = amount_usdc / artist_tokens_price;
    proposal.artist_tokens_sold += tokens_bought;
    proposal.usdc_collected += amount_usdc;

    // Optional: mark early access
    if is_campaign_purchase {
        proposal.early_access = true; // only first campaign buyers
    }

    Ok(tokens_bought)
}
