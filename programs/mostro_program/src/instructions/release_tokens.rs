use anchor_lang::prelude::*;
use crate::state::Proposal;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct ReleaseTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    /// CHECK: This is the artist's wallet. We don't need to deserialize it, just transfer lamports.
    #[account(mut)]
    pub artist_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn release_tokens_handler(
    ctx: Context<ReleaseTokens>,
    artist_token_price: u64,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    match proposal.status {
        1 => { // Approved
            if proposal.milestone_reached {
                // Transfer all USDC to artist
                **ctx.accounts.artist_wallet.try_borrow_mut_lamports()? += proposal.usdc_collected;
            } else {
                // Calculate remaining tokens equivalent to USDC
                let tokens_equivalent = proposal.usdc_collected / artist_token_price;

                // Transfer USDC
                **ctx.accounts.artist_wallet.try_borrow_mut_lamports()? += proposal.usdc_collected;

                // Off-chain: mint `tokens_equivalent` artist tokens to artist wallet
                // (handled outside the program)
            }
        },
        2 => { // Rejected
            // Refund USDC to artist (or backers? depending on logic)
            **ctx.accounts.artist_wallet.try_borrow_mut_lamports()? += proposal.usdc_collected;
        },
        _ => return Err(ErrorCode::ProposalNotFinalized.into()),
    }

    // Reset counters
    proposal.usdc_collected = 0;
    proposal.artist_tokens_sold = 0;

    Ok(())
}
