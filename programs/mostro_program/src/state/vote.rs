use anchor_lang::prelude::*;

/// Minimal account just to prevent double voting
#[account]
pub struct VoteMarker {
    // Nothing else needed; existence is enough
}
