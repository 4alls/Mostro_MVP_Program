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
pub use create_artist::*;             // exports `create_artist_handler` and `CreateArtist`
pub use create_config::*;             // exports `create_config_handler` and `CreateConfig`
pub use create_proposal::*;           // exports `create_proposal_handler` and `CreateProposal`
pub use vote_proposal::*;             // exports `vote_proposal_handler` and `VoteProposal`
pub use finalize_proposal::*;         // exports `finalize_proposal_handler` and `FinalizeProposal`
pub use buy_tokens_for_proposal::*;   // exports `buy_tokens_for_proposal_handler` and `BuyTokensForProposal`
pub use release_tokens::*;            // exports `release_tokens_handler` and `ReleaseTokens`
