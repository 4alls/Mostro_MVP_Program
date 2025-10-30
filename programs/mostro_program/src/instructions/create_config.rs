use anchor_lang::prelude::*;
use crate::state::Config;
use crate::error::ErrorCode;

/// --------------------------------------------
/// Accounts Context: CreateConfig
/// --------------------------------------------
/// This context initializes the global configuration PDA.
/// It stores protocol-wide parameters like fee percentages
/// and external service wallets.
/// Every artist, proposal, or campaign uses that same config PDA to read shared parameters (like fee percentages or the pump.fun wallet).
/// Only the admin can create this PDA, and it’s initialized once.
#[derive(Accounts)]
pub struct CreateConfig<'info> {
    /// The admin signer — responsible for initializing the config
    /// and paying rent for the PDA creation.
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The global configuration PDA.
    ///
    /// - Seeds: fixed "global_config" to ensure there’s only one.
    /// - Payer: admin funds the rent for this account.
    /// - Space: computed via Config::space() for safety and readability.
    /// - The bump is automatically derived and stored by Anchor.
    #[account(
        init,
        payer = admin,
        space = Config::space(),
        seeds = [b"global_config"],
        bump
    )]
    pub config: Account<'info, Config>,

    /// The Solana system program — required for creating new accounts.
    pub system_program: Program<'info, System>,
}

/// --------------------------------------------
/// Handler: create_config_handler
/// --------------------------------------------
/// Initializes the protocol configuration by setting:
/// - the fee split between the artist and the platform
/// - the admin wallet (creator)
/// - the pump.fun service wallet
///
/// Constraints:
/// - The sum of artist and Mostro percentages must not exceed 100.
/// - This function can only be run once per deployment, unless the
///   config account is closed and re-created.
pub fn create_config_handler(
    ctx: Context<CreateConfig>,
    percentage_artist: u8,
    percentage_mostro: u8,
    pump_fun_service_wallet: Pubkey,
) -> Result<()> {
    // Ensure valid fee distribution (cannot exceed 100%)
    require!(
        percentage_artist + percentage_mostro <= 100,
        ErrorCode::InvalidPercentage
    );

    // Initialize config data
    let config = &mut ctx.accounts.config;
    config.percentage_artist = percentage_artist;
    config.percentage_mostro = percentage_mostro;
    config.admin_wallet = ctx.accounts.admin.key();
    config.pump_fun_service_wallet = pump_fun_service_wallet;

    Ok(())
}
