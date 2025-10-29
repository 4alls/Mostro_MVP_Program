// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anchor_lang::prelude::*;
//     use anchor_lang::solana_program::pubkey::Pubkey;
//     use anchor_lang::prelude::ProgramError;

//     // Mock accounts for testing
//     #[derive(Default)]
//     struct MockAccounts {
//         config: Account<Config>,
//         admin: Pubkey,
//     }

//     #[account]
//     #[derive(Default)]
//     pub struct Config {
//         pub percentage_artist: u8,
//         pub percentage_mostro: u8,
//         pub admin_wallet: Pubkey,
//         pub pump_fun_service_wallet: Pubkey,
//     }

//     #[derive(Accounts)]
//     pub struct CreateConfig<'info> {
//         #[account(init, payer = admin, space = 8 + 32 + 32 + 1 + 1)]
//         pub config: Account<'info, Config>,
//         #[account(mut)]
//         pub admin: Signer<'info>,
//         pub system_program: Program<'info, System>,
//     }

//     #[test]
//     fn test_create_config_success() {
//         let mut accounts = MockAccounts::default();
//         accounts.admin = Pubkey::new_unique();

//         // Create a dummy context
//         let ctx = Context::new(
//             Program::default(),
//             &mut [
//                 (&mut accounts.config, true), // is_signer
//             ],
//             &mut [
//                 (&accounts.admin, true),
//             ],
//             None,
//         );

//         // Call handler
//         let percentage_artist = 40;
//         let percentage_mostro = 50;
//         let pump_wallet = Pubkey::new_unique();

//         let res = create_config_handler(ctx, percentage_artist, percentage_mostro, pump_wallet);
//         assert!(res.is_ok());

//         // Check values
//         assert_eq!(accounts.config.percentage_artist, 40);
//         assert_eq!(accounts.config.percentage_mostro, 50);
//         assert_eq!(accounts.config.pump_fun_service_wallet, pump_wallet);
//         assert_eq!(accounts.config.admin_wallet, accounts.admin);
//     }

//     #[test]
//     fn test_create_config_invalid_percentage() {
//         let mut accounts = MockAccounts::default();
//         accounts.admin = Pubkey::new_unique();

//         let ctx = Context::new(
//             Program::default(),
//             &mut [
//                 (&mut accounts.config, true), // is_signer
//             ],
//             &mut [
//                 (&accounts.admin, true),
//             ],
//             None,
//         );

//         // Sum > 100
//         let res = create_config_handler(ctx, 60, 50, Pubkey::new_unique());
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err().to_string(), ErrorCode::InvalidPercentage.to_string());
//     }
// }
