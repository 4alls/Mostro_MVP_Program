use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(Accounts)]
pub struct CreateConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // Admin paying for PDA creation

    #[account(
        init,
        payer = admin,
        space = Config::space(),
        seeds = [b"global_config"],
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

pub fn create_config_handler(
    ctx: Context<CreateConfig>,
    percentage_artist: u8,
    percentage_mostro: u8,
    pump_fun_service_wallet: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    config.percentage_artist = percentage_artist;
    config.percentage_mostro = percentage_mostro;
    config.admin_wallet = *ctx.accounts.admin.key; 
    config.pump_fun_service_wallet = pump_fun_service_wallet;

    // bump is automatically assigned by Anchor
    Ok(())
}

