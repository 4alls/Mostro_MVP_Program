//Needs to be deleted because the streamflow vesting will be used

use anchor_lang::prelude::*;
use crate::state::Proposal;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    // The platform admin triggers the release of funds.
    // Only the admin should call this to prevent unauthorized transfers.
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    // The proposal being finalized. Tracks USDC collected, tokens sold, status, etc.
    /// CHECK: This is the artist's wallet. We don't need to deserialize it, just transfer lamports.
    #[account(mut)]
    pub artist_wallet: AccountInfo<'info>,
    // Unsafe AccountInfo because we just need to transfer lamports (SOL or USDC if wrapped in SPL token).
    // No checks needed here; only the admin can call, so this is safe.
    pub system_program: Program<'info, System>,
    // Required to perform lamport transfers on-chain
}

pub fn release_tokens_handler(ctx: Context<ReleaseTokens>, artist_token_price: u64) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    match proposal.status {
        1 => {
            // Approved
            if proposal.milestone_reached {
                // If milestone is reached, release all USDC collected directly to the artist
                **ctx.accounts.artist_wallet.try_borrow_mut_lamports()? += proposal.usdc_collected;
            } else {
                // Otherwise, calculate how many artist tokens should be minted for the collected USDC
                // WE ARE CURRENTLY NOT USING THIS VALUE
                let _tokens_equivalent = proposal.usdc_collected / artist_token_price;

                // Transfer USDC to artist
                **ctx.accounts.artist_wallet.try_borrow_mut_lamports()? += proposal.usdc_collected;

                // Tokens are minted off-chain according to `tokens_equivalent` (not handled on-chain)
            }
        }
        2 => {
            // Rejected
            // Refund USDC back to artist (or possibly backers; depends on your business logic)
            **ctx.accounts.artist_wallet.try_borrow_mut_lamports()? += proposal.usdc_collected;
        }
        _ => return Err(ErrorCode::ProposalNotFinalized.into()),
        // If proposal is not yet finalized, prevent releasing funds
    }

    // Reset counters to prevent double distribution
    proposal.usdc_collected = 0;
    proposal.artist_tokens_sold = 0;

    Ok(())
}
