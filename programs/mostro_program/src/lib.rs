use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

// -----------------------------
// Declare modules
// -----------------------------
pub mod instructions;
pub mod state;
pub mod error;

// Optional future modules
// pub mod constants;

// Bring all instruction handlers into program scope
pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("5PdCQfcDhiZCyQs6wx3rz1DZ2c1nkcxAXPYpLdmPHWrY");

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

    pub fn vote_on_proposal(
        ctx: Context<VoteOnProposal>,
        name: String,
        proposal_id: u64,
        vote_choice: bool,
    ) -> Result<()> {
        vote_on_proposal_handler(ctx, name, proposal_id, vote_choice)
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
        release_tokens_to_artist(ctx, amount)
    }
}
