#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;
// Compile this code only when the "anchor" feature is enabled (ignored in pure Cargo builds)
#[cfg(feature = "anchor")] 
use anchor_lang::solana_program::pubkey::Pubkey;

// -----------------------------
// Declare modules
// -----------------------------
pub mod instructions;
pub mod state;
pub mod error;

// Bring all instruction handlers into program scope
use instructions::*;

declare_id!("2SYi3NFHTnCXHEzxNpa8nEyehkmZPyikbCarmxngSdTn");

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
        create_config_handler(
            ctx,
            percentage_artist,
            percentage_mostro,
            pump_fun_service_wallet,
        )
    }

    // -----------------------------
    // Governance / Proposals
    // -----------------------------
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        name: String,
        proposal_id: u64,
        title: String,
        number_of_tokens: u64,
    ) -> Result<()> {
        create_proposal_handler(ctx, name, proposal_id, title, number_of_tokens)
    }

    pub fn vote_proposal(
        ctx: Context<VoteProposal>,
        name: String,
        proposal_id: u64,
        vote_choice: bool,
    ) -> Result<()> {
        vote_proposal_handler(ctx, name, proposal_id, vote_choice)
    }

    // -----------------------------
    // Artist Management
    // -----------------------------
    pub fn create_artist(
        ctx: Context<CreateArtist>,
        metadata_uri: String,
        percentage_artist: Option<u8>,
        percentage_mostro: Option<u8>,
    ) -> Result<()> {
        create_artist_handler(ctx, metadata_uri, percentage_artist, percentage_mostro)
    }

    pub fn release_tokens_to_artist(
        ctx: Context<ReleaseTokens>,
        amount: u64,
    ) -> Result<()> {
        release_tokens_to_artist_handler(ctx, amount)
    }
}

// ---------------------------------------
// Add a Cargo-only entrypoint for manual testing
// ---------------------------------------
#[cfg(not(feature = "anchor"))]
mod manual_entrypoint {
    use anchor_lang::solana_program::entrypoint;
    use anchor_lang::solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
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

// -----------------------------
// Include tests (integration / unit)
// -----------------------------
#[cfg(test)]
mod tests;