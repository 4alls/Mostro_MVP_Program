// -----------------------------
// Instruction modules
// -----------------------------
pub mod create_artist;
pub mod create_config;
pub mod create_proposal;
pub mod vote_on_proposal;
pub mod release_tokens;

// -----------------------------
// Instruction handlers & accounts
// -----------------------------
// This allows `lib.rs` to do `use instructions::*;` and access everything
pub use create_artist::*; // create_artist_handler, CreateArtist
pub use create_config::*; // create_config_handler, CreateConfig
pub use create_proposal::*; // create_proposal_handler, CreateProposal
pub use vote_on_proposal::*; // vote_on_proposal_handler, VoteOnProposal
pub use release_tokens::*; // release_tokens_to_artist_handler, ReleaseTokensToArtist
