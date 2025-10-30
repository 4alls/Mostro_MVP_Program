// -----------------------------
// Instruction modules
// -----------------------------
pub mod create_artist;
pub mod create_config;
pub mod create_proposal;
pub mod vote_proposal;
pub mod finalize_proposal;
pub mod buy_tokens_for_proposal;
pub mod release_tokens;

// -----------------------------
// Instruction handlers & accounts
// -----------------------------
// -----------------------------
// Instruction handlers & accounts
// -----------------------------
pub use create_artist::{create_artist_handler, CreateArtist};
pub use create_config::{create_config_handler, CreateConfig};
pub use create_proposal::{create_proposal_handler, CreateProposal};
pub use vote_proposal::{vote_proposal_handler, VoteProposal};
pub use finalize_proposal::{finalize_proposal_handler, FinalizeProposal};
pub use buy_tokens_for_proposal::{buy_tokens_for_proposal_handler, BuyTokensForProposal};
pub use release_tokens::{release_tokens_handler, ReleaseTokens};        