use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub percentage_artist: u8,
    pub percentage_mostro: u8,
    pub admin_wallet: Pubkey,
    pub pump_fun_service_wallet: Pubkey,
}

impl Config {
    pub fn space() -> usize {
        8 +   // discriminator
        1 +   // percentage_artist
        1 +   // percentage_mostro
        32 +  // admin_wallet
        32    // pump_fun_service_wallet
    }
}
