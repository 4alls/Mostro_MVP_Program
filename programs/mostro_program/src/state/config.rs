use anchor_lang::prelude::*;

/// Global configuration for the Mostro protocol
#[account]
pub struct Config {
    pub percentage_artist: u8,              // % allocated to artists
    pub percentage_mostro: u8,              // % allocated to Mostro platform
    pub admin_wallet: Pubkey,               // Program admin wallet
    pub pump_fun_service_wallet: Pubkey,    // Wallet used for Pump.fun token creation
    pub bump: u8,                           // PDA bump for signing
}

impl Config {
    pub fn space() -> usize {
        8 +  // discriminator
        1 +  // percentage_artist
        1 +  // percentage_mostro
        32 + // admin_wallet
        32 + // pump_fun_service_wallet
        1    // bump
    }
}
