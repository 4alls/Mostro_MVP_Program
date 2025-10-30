#![allow(unexpected_cfgs)] // suppress Anchor macro warnings

use anchor_lang::prelude::*;

// -----------------------------
// Declare modules first
// -----------------------------
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("2SYi3NFHTnCXHEzxNpa8nEyehkmZPyikbCarmxngSdTn");

// Explicit imports for #[program]
use instructions::create_config::{create_config_handler, CreateConfig};
use instructions::create_artist::{create_artist_handler, CreateArtist};
use instructions::create_proposal::{create_proposal_handler, CreateProposal};
use instructions::buy_tokens_for_proposal::{buy_tokens_for_proposal_handler, BuyTokensForProposal};
use instructions::vote_proposal::{vote_proposal_handler, VoteProposal};
use instructions::finalize_proposal::{finalize_proposal_handler, FinalizeProposal};
use instructions::release_tokens::{release_tokens_handler, ReleaseTokens};

/// -----------------------------
/// Anchor program entrypoint
/// -----------------------------
#[cfg(feature = "anchor")]
#[program]
pub mod mostro_program {
    use super::*;

    // -----------------------------
    // Global Config
    // -----------------------------
    pub fn create_config(
        ctx: Context<CreateConfig>,
        percentage_artist: u8,
        percentage_mostro: u8,
        pump_fun_service_wallet: Pubkey,
    ) -> Result<()> {
        create_config_handler(ctx, percentage_artist, percentage_mostro, pump_fun_service_wallet)
    }

    // -----------------------------
    // Governance / Proposals
    // -----------------------------
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        number_of_tokens: u64,
        milestone_reached: bool,
        early_access: bool,
        bump: u8,
    ) -> Result<()> {
        create_proposal_handler(
            ctx,
            title,
            description,
            number_of_tokens,
            milestone_reached,
            early_access,
            bump,
        )
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

    // -----------------------------
    // Artist Management
    // -----------------------------
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

    pub fn release_tokens_to_artist(
        ctx: Context<ReleaseTokens>,
        artist_token_price: u64,
    ) -> Result<()> {
        release_tokens_handler(ctx, artist_token_price)
    }
}

/// -----------------------------
/// Manual cargo-only entrypoint
/// -----------------------------
#[cfg(feature = "manual")]
mod manual_entrypoint {
    use anchor_lang::solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    };

    entrypoint!(process_instruction);

    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        Ok(())
    }
}

/// -----------------------------
/// Include tests
/// -----------------------------
#[cfg(test)]
mod tests;
