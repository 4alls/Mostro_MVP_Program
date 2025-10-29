use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};
use crate::state::Proposal;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct BuyTokensForProposal<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>, // User buying tokens

    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    /// CHECK: PDA holding vested artist tokens
    #[account(mut)]
    pub artist_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>, // Buyer receives artist tokens

    #[account(mut)]
    pub buyer_usdc_account: Account<'info, TokenAccount>, // Buyer pays USDC

    #[account(mut)]
    pub usdc_vault: Account<'info, TokenAccount>, // PDA USDC account to receive funds

    pub token_program: Program<'info, Token>,
}

pub fn buy_tokens_for_proposal_handler(
    ctx: Context<BuyTokensForProposal>,
    amount_usdc: u64,
    artist_tokens_price: u64,
    is_campaign_purchase: bool, // determines early access
    vault_bump: u8, // bump for artist_vault PDA
) -> Result<u64> {
    let proposal = &mut ctx.accounts.proposal;

    // Calculate number of artist tokens to give
    let tokens_bought = amount_usdc
        .checked_div(artist_tokens_price)
        .ok_or(ErrorCode::Overflow)?;

    // --- Step 1: Transfer USDC from buyer to PDA vault ---
    let cpi_accounts_usdc = Transfer {
        from: ctx.accounts.buyer_usdc_account.to_account_info(),
        to: ctx.accounts.usdc_vault.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
    };
    let cpi_ctx_usdc = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_usdc);
    token::transfer(cpi_ctx_usdc, amount_usdc)?;

    // --- Step 2: Transfer artist tokens from PDA to buyer ---
    let proposal_key = proposal.key(); // 
    let seeds = &[b"artist_vault", proposal_key.as_ref(), &[vault_bump]];
    let signer = &[&seeds[..]];

    let cpi_accounts_artist = Transfer {
        from: ctx.accounts.artist_vault.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.artist_vault.to_account_info(),
    };
    let cpi_ctx_artist = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts_artist,
        signer,
    );
    token::transfer(cpi_ctx_artist, tokens_bought)?;

    // --- Step 3: Update proposal state ---
    proposal.artist_tokens_sold = proposal.artist_tokens_sold
        .checked_add(tokens_bought)
        .ok_or(ErrorCode::Overflow)?;
    proposal.usdc_collected = proposal.usdc_collected
        .checked_add(amount_usdc)
        .ok_or(ErrorCode::Overflow)?;

    // Optional: mark early access
    if is_campaign_purchase {
        proposal.early_access = true;
    }

    Ok(tokens_bought)
}
