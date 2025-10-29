#[account]
pub struct Vote {
    pub voter: Pubkey,
    pub proposal: Pubkey,
    pub vote_yes: bool,
    pub token_weight: u64,
}

impl Vote {
    pub fn space() -> usize {
        8 + 32 + 32 + 1 + 8
    }
}
