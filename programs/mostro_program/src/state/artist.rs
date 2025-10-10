#![allow(unexpected_cfgs)] // Suppress warnings from Anchor macros (e.g., #[cfg(anchor-debug)])

use anchor_lang::prelude::*;
use serde::{Serialize, Deserialize};

/// ---------------------------------------------
/// Artist Account
/// ---------------------------------------------
/// Represents one registered artist in the protocol.
/// Each artist has:
///  - Their wallet address
///  - A Pump.fun token mint (created externally)
///  - A metadata URI for frontend use
///  - Platform + artist allocation percentages
///  - A link to the global config PDA
///  - A bump for PDA derivation
///
/// The corresponding token vault (holding the artist’s tokens)
/// will be managed in a separate instruction.
/// ---------------------------------------------

/// Maximum length for artist metadata URI
pub const MAX_URI_LEN: usize = 200;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[account]
pub struct Artist {
    /// Wallet address of the artist (not necessarily the vault owner)
    pub artist_wallet: Pubkey,

    /// Mint address of the artist’s Pump.fun token
    pub pump_token_mint: Pubkey,

    /// Off-chain metadata URI (e.g. IPFS or JSON metadata)
    pub metadata_uri: String,

    /// Percentage of proceeds allocated to the artist
    pub percentage_artist: u8,

    /// Percentage of proceeds allocated to the platform (Mostro)
    pub percentage_mostro: u8,

    /// Reference to the global configuration PDA
    pub global_config: Pubkey,

    /// The vault holding the artist's tokens
    pub artist_vault: Pubkey,

    /// PDA bump (for deriving the artist account deterministically)
    pub bump: u8,
}

impl Artist {
    /// ---------------------------------------------
    /// Returns the required account size in bytes.
    ///
    /// Note:
    ///  - Strings in Anchor are stored with a 4-byte length prefix.
    ///  - Always include 8 bytes for the Anchor discriminator.
    /// ---------------------------------------------
    pub fn space() -> usize {
        8 +   // discriminator
        32 +  // artist_wallet
        32 +  // pump_token_mint
        4 + MAX_URI_LEN + // metadata_uri (4-byte length prefix + string)
        1 +   // percentage_artist
        1 +   // percentage_mostro
        32 +  // global_config
        32 +  // artist_vault
        1     // bump
    }
}
