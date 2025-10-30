use anchor_lang::prelude::*;

// Re-export our instruction modules
pub mod error;
pub mod state;
pub mod instructions;

use instructions::*;
use state::*;

/// -----------------------------
/// Program Declaration
/// -----------------------------
declare_id!("33pbYVD5NnHpnAc6kfSjiymWUt2Ttgp7NMTZMaY621c7"); // replace this with actual id

#[program]
pub mod mostro_program {
    use super::*;

    // Each instruction is exposed to Anchor clients here

    pub fn create_config(
        ctx: Context<CreateConfig>,
        percentage_artist: u8,
        percentage_mostro: u8,
        pump_fun_service_wallet: Pubkey,
    ) -> Result<()> {
        create_config_handler(ctx, percentage_artist, percentage_mostro, pump_fun_service_wallet)
    }

    pub fn create_artist(
        ctx: Context<CreateArtist>,
        artist_name: String,
        image: String,
        latest_single_title: String,
        latest_single_duration: String,
        mint: Pubkey,
        total_tokens: u64,
    ) -> Result<()> {
        create_artist_handler(ctx, artist_name, image, latest_single_title, latest_single_duration, mint, total_tokens)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        number_of_tokens: u64,
        milestone_reached: bool,
        early_access: bool,
        bump: u8,
    ) -> Result<()> {
        create_proposal_handler(ctx, title, description, number_of_tokens, milestone_reached, early_access, bump)
    }

    pub fn vote_proposal(
        ctx: Context<VoteProposal>,
        vote_yes: bool,
        voter_token_balance: u64,
    ) -> Result<()> {
        vote_proposal_handler(ctx, vote_yes, voter_token_balance)
    }

    pub fn finalize_proposal(ctx: Context<FinalizeProposal>) -> Result<()> {
        finalize_proposal_handler(ctx)
    }

    pub fn buy_tokens_for_proposal(
        ctx: Context<BuyTokensForProposal>,
        amount_usdc: u64,
        artist_tokens_price: u64,
        is_campaign_purchase: bool,
        vault_bump: u8,
    ) -> Result<u64> {
        buy_tokens_for_proposal_handler(ctx, amount_usdc, artist_tokens_price, is_campaign_purchase, vault_bump)
    }

    pub fn release_tokens(
        ctx: Context<ReleaseTokens>,
        artist_token_price: u64,
    ) -> Result<()> {
        release_tokens_handler(ctx, artist_token_price)
    }
}
