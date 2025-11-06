use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};
use crate::state::Perk;

#[derive(Accounts)]
pub struct BuyPerks<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, 

    #[account(mut)]
    pub perk: Account<'info, Perk>,

    #[account(mut)]
    pub artist_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>, 

    #[account(mut)]
    pub buyer_usdc_account: Account<'info, TokenAccount>, 

    #[account(mut)]
    pub usdc_vault: Account<'info, TokenAccount>, 

    pub token_program: Program<'info, Token>,
}

pub fn buy_perks_USDC(
    ctx: Context<BuyPerks>,
    perk_title: String,
) -> Result<> {
    let perk = &mut ctx.accounts.perk;
    let buyer_usdc_account = &mut ctx.accounts.buyer_usdc_account;
    let usdc_vault = &mut ctx.accounts.usdc_vault;
    let amount_usdc = perk.price_in_usdc;

    let cpi_accounts_usdc = Transfer {
        from: buyer_usdc_account.to_account_info(),
        to: usdc_vault.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    let cpi_ctx_usdc = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_usdc);
    token::transfer(cpi_ctx_usdc, amount_usdc)?;

    Ok(())
}

pub fn buy_perks_artist_tokens(
    ctx: Context<BuyPerks>,
    perk_title: String,
) -> Result<> {
    let perk = &mut ctx.accounts.perk;
    let buyer_token_account = &mut ctx.accounts.buyer_token_account;
    let artist_vault = &mut ctx.accounts.artist_vault;
    let amount_tokens = perk.price_in_tokens;

    let cpi_accounts_tokens = Transfer {
        from: buyer_token_account.to_account_info(),
        to: artist_vault.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    let cpi_ctx_tokens = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_tokens);
    token::transfer(cpi_ctx_tokens, amount_tokens)?;

    Ok(())
}