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
pub mod create_perk;
pub mod create_vesting_stream;
// -----------------------------
// Instruction handlers & accounts
// -----------------------------
pub use create_artist::*;
pub use create_config::*;
pub use create_proposal::*;
pub use vote_proposal::*;
pub use finalize_proposal::*;
pub use buy_tokens_for_proposal::*;
pub use release_tokens::*;   
pub use create_perk::*;
pub use create_vesting_stream::*;
