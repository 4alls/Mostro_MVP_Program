// #[cfg(test)]
// mod tests {
//     use crate::error::ErrorCode;
//     use crate::state::Proposal;
//     use anchor_lang::prelude::Pubkey;
//     use anchor_lang::{AccountDeserialize, AccountSerialize};
//     use bincode;
//     use std::collections::HashMap;
//     use std::collections::HashSet;

//     // -----------------------------
//     // Mock Error Enum
//     // -----------------------------
//     #[derive(Debug, PartialEq)]
//     enum TestError {
//         Unauthorized,
//         InvalidPercentage,
//     }

//     // -----------------------------
//     // State Structs
//     // -----------------------------
//     #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
//     struct Config {
//         percentage_artist: u8,
//         percentage_mostro: u8,
//         admin_wallet: Pubkey,
//         pump_fun_service_wallet: Pubkey,
//         bump: u8,
//     }

//     #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
//     struct LatestSingle {
//         title: String,
//         duration: String,
//     }

//     #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
//     struct Artist {
//         name: String,
//         image: String,
//         latest_single: LatestSingle,
//         mint: Pubkey,
//         total_tokens: u64,
//         campaign_tokens_sold: u64,
//     }

//     // -----------------------------
//     // PDA Helpers
//     // -----------------------------
//     fn get_config_pda(program_id: &Pubkey) -> (Pubkey, u8) {
//         Pubkey::find_program_address(&[b"global_config"], program_id)
//     }

//     fn get_artist_pda(program_id: &Pubkey, artist_name: &str) -> (Pubkey, u8) {
//         Pubkey::find_program_address(&[b"artist", artist_name.as_bytes()], program_id)
//     }

//     fn get_proposal_pda(program_id: &Pubkey, artist: &Pubkey, proposal_id: u64) -> (Pubkey, u8) {
//         Pubkey::find_program_address(
//             &[
//                 b"artist_proposal",
//                 artist.as_ref(),
//                 &proposal_id.to_le_bytes(),
//             ],
//             program_id,
//         )
//     }

//     // -----------------------------
//     // Config Tests
//     // -----------------------------
//     #[test]
//     fn test_create_config_admin_only() {
//         let admin = Pubkey::new_unique();
//         let unauthorized = Pubkey::new_unique();
//         let program_id = Pubkey::new_unique();
//         let (_config_pda, bump) = get_config_pda(&program_id);

//         let create_config = |caller: Pubkey,
//                              admin_wallet: Pubkey,
//                              percentage_artist: u8,
//                              percentage_mostro: u8,
//                              pump_wallet: Pubkey,
//                              bump: u8|
//          -> Result<Config, TestError> {
//             if caller != admin_wallet {
//                 return Err(TestError::Unauthorized);
//             }
//             if percentage_artist + percentage_mostro > 100 {
//                 return Err(TestError::InvalidPercentage);
//             }
//             Ok(Config {
//                 percentage_artist,
//                 percentage_mostro,
//                 admin_wallet,
//                 pump_fun_service_wallet: pump_wallet,
//                 bump,
//             })
//         };

//         // Admin should succeed
//         let res_ok = create_config(admin, admin, 10, 5, Pubkey::new_unique(), bump);
//         assert!(res_ok.is_ok());

//         // Unauthorized should fail
//         let res_err = create_config(unauthorized, admin, 10, 5, Pubkey::new_unique(), bump);
//         assert!(res_err.is_err());
//         assert_eq!(res_err.unwrap_err(), TestError::Unauthorized);
//     }

//     #[test]
//     fn test_config_serialization() {
//         let admin = Pubkey::new_unique();
//         let pump = Pubkey::new_unique();
//         let program_id = Pubkey::new_unique();
//         let (_config_pda, bump) = get_config_pda(&program_id);

//         let config = Config {
//             percentage_artist: 10,
//             percentage_mostro: 5,
//             admin_wallet: admin,
//             pump_fun_service_wallet: pump,
//             bump,
//         };

//         let encoded = bincode::serialize(&config).expect("Serialization failed");
//         let decoded: Config = bincode::deserialize(&encoded).expect("Deserialization failed");
//         assert_eq!(decoded, config);
//     }

//     // -----------------------------
//     // Artist Tests
//     // -----------------------------
//     #[test]
//     fn test_artist_pda_derivation() {
//         let program_id = Pubkey::new_unique();
//         let artist_name = "CoolArtist";
//         let (pda1, bump1) = get_artist_pda(&program_id, artist_name);
//         let (pda2, bump2) = get_artist_pda(&program_id, artist_name);

//         assert_eq!(pda1, pda2);
//         assert_eq!(bump1, bump2);
//     }

//     #[test]
//     fn test_artist_creation_and_storage() {
//         let program_id = Pubkey::new_unique();
//         let artist_name = "MyArtist";
//         let (artist_pda, _bump) = get_artist_pda(&program_id, artist_name);

//         let artist = Artist {
//             name: artist_name.to_string(),
//             image: "https://image.png".to_string(),
//             latest_single: LatestSingle {
//                 title: "Hit Single".to_string(),
//                 duration: "3:30".to_string(),
//             },
//             mint: Pubkey::new_unique(),
//             total_tokens: 1000,
//             campaign_tokens_sold: 0,
//         };

//         // Encode / decode
//         let encoded = bincode::serialize(&artist).expect("Serialization failed");
//         let decoded: Artist = bincode::deserialize(&encoded).expect("Deserialization failed");
//         assert_eq!(decoded, artist);

//         // Simulate PDA check
//         let (expected_pda, _bump) = get_artist_pda(&program_id, &artist_name);
//         assert_eq!(expected_pda, artist_pda);
//     }

//     #[test]
//     fn test_reject_duplicate_artist() {
//         let mut registry: HashMap<Pubkey, Artist> = HashMap::new();
//         let artist_name = "Duplicate";
//         let program_id = Pubkey::new_unique();
//         let (artist_pda, _bump) = get_artist_pda(&program_id, artist_name);

//         let artist1 = Artist {
//             name: artist_name.to_string(),
//             image: "https://one.com".to_string(),
//             latest_single: LatestSingle {
//                 title: "Single1".to_string(),
//                 duration: "2:30".to_string(),
//             },
//             mint: Pubkey::new_unique(),
//             total_tokens: 500,
//             campaign_tokens_sold: 0,
//         };

//         registry.insert(artist_pda, artist1);

//         let artist_duplicate = Artist {
//             name: artist_name.to_string(),
//             image: "https://two.com".to_string(),
//             latest_single: LatestSingle {
//                 title: "Single2".to_string(),
//                 duration: "3:00".to_string(),
//             },
//             mint: Pubkey::new_unique(),
//             total_tokens: 700,
//             campaign_tokens_sold: 0,
//         };

//         let res = registry.insert(artist_pda, artist_duplicate);
//         assert!(
//             res.is_some(),
//             "Duplicate artist insertion should be detected"
//         );
//     }

//     // -----------------------------
//     // Proposal Tests
//     // -----------------------------
//     #[test]
//     fn test_proposal_pda_derivation() {
//         let program_id = Pubkey::new_unique();
//         let artist = Pubkey::new_unique();
//         let proposal_id = 42;

//         let (pda1, bump1) = get_proposal_pda(&program_id, &artist, proposal_id);
//         let (pda2, bump2) = get_proposal_pda(&program_id, &artist, proposal_id);

//         assert_eq!(pda1, pda2);
//         assert_eq!(bump1, bump2);
//     }

//     #[test]
//     fn test_proposal_creation_and_serialization() {
//         let artist = Pubkey::new_unique();
//         let creator = Pubkey::new_unique();

//         let proposal = Proposal {
//             artist,
//             creator,
//             title: "My Proposal".to_string(),
//             description: "https://ipfs.io/myproposal".to_string(),
//             number_of_tokens: 1000,
//             yes_votes: 0,
//             no_votes: 0,
//             start_date: 1_700_000_000,
//             end_date: 1_700_000_100,
//             status: 0,
//             milestone_reached: false,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 255,
//         };

//         // Serialize using Anchor's AccountSerialize
//         let mut buf = Vec::with_capacity(Proposal::space());
//         proposal
//             .try_serialize(&mut buf)
//             .expect("Serialization failed");

//         // Deserialize using Anchor's AccountDeserialize
//         let decoded =
//             Proposal::try_deserialize(&mut buf.as_slice()).expect("Deserialization failed");

//         assert_eq!(decoded, proposal);
//     }

//     #[test]
//     fn test_reject_duplicate_proposals() {
//         let mut proposals: HashMap<(Pubkey, u64), Proposal> = HashMap::new();
//         let artist = Pubkey::new_unique();
//         let creator = Pubkey::new_unique();
//         let proposal_id = 0;

//         let proposal1 = Proposal {
//             artist,
//             creator,
//             title: "First Proposal".to_string(),
//             description: "https://example.com".to_string(),
//             number_of_tokens: 1000,
//             yes_votes: 0,
//             no_votes: 0,
//             start_date: 1_700_000_000,
//             end_date: 1_700_000_100,
//             status: 0,
//             milestone_reached: false,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 255,
//         };

//         // First insert should succeed
//         assert!(proposals
//             .insert((artist, proposal_id), proposal1.clone())
//             .is_none());

//         // Duplicate proposal should be detected
//         let proposal_duplicate = Proposal {
//             title: "Duplicate".to_string(),
//             ..proposal1
//         };
//         assert!(proposals
//             .insert((artist, proposal_id), proposal_duplicate)
//             .is_some());
//     }

//     #[test]
//     fn test_invalid_proposal_dates() {
//         fn validate_proposal(proposal: &Proposal) -> Result<(), &'static str> {
//             if proposal.end_date < proposal.start_date {
//                 return Err("End date cannot be before start date");
//             }
//             Ok(())
//         }

//         let proposal = Proposal {
//             artist: Pubkey::new_unique(),
//             creator: Pubkey::new_unique(),
//             title: "Invalid Dates".to_string(),
//             description: "Desc".to_string(),
//             number_of_tokens: 100,
//             yes_votes: 0,
//             no_votes: 0,
//             start_date: 10,
//             end_date: 5,
//             status: 0,
//             milestone_reached: false,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 0,
//         };

//         assert!(validate_proposal(&proposal).is_err());
//     }

//     /// Mock Clock
//     struct MockClock {
//         pub unix_timestamp: i64,
//     }

//     impl MockClock {
//         fn get() -> Self {
//             Self {
//                 unix_timestamp: 1_700_000_050,
//             }
//         }
//     }

//     /// Mock VoteProposal Context
//     struct MockVoteCtx<'a> {
//         pub voter: Pubkey,
//         pub proposal: &'a mut Proposal,
//         pub vote_marker_registry: &'a mut HashSet<(Pubkey, Pubkey)>, // (proposal, voter)
//     }

//     impl<'a> MockVoteCtx<'a> {
//         fn vote(&mut self, vote_yes: bool, voter_token_balance: u64) -> Result<(), ErrorCode> {
//             let clock = MockClock::get();

//             // Step 1: Ensure voting is active
//             if clock.unix_timestamp > self.proposal.end_date || self.proposal.status != 0 {
//                 return Err(ErrorCode::VotingEnded);
//             }

//             // Step 2: Prevent double voting
//             if !self
//                 .vote_marker_registry
//                 .insert((self.proposal.artist, self.voter))
//             {
//                 return Err(ErrorCode::AlreadyVoted);
//             }

//             // Step 3: Increment votes
//             if vote_yes {
//                 self.proposal.yes_votes += voter_token_balance;
//             } else {
//                 self.proposal.no_votes += voter_token_balance;
//             }

//             // Step 4: Early finalize
//             if self.proposal.milestone_reached
//                 && self.proposal.yes_votes > self.proposal.number_of_tokens / 2
//             {
//                 self.proposal.status = 1;
//                 self.proposal.end_date = clock.unix_timestamp;
//             }

//             Ok(())
//         }
//     }

//     #[test]
//     fn test_vote_proposal_basic() {
//         let artist = Pubkey::new_unique();
//         let creator = Pubkey::new_unique();
//         let mut proposal = Proposal {
//             artist,
//             creator,
//             title: "Test Proposal".to_string(),
//             description: "Desc".to_string(),
//             number_of_tokens: 1000,
//             yes_votes: 0,
//             no_votes: 0,
//             start_date: 1_700_000_000,
//             end_date: 1_700_000_100,
//             status: 0,
//             milestone_reached: false,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 255,
//         };

//         let voter = Pubkey::new_unique();
//         let mut vote_registry = HashSet::new();

//         let mut ctx = MockVoteCtx {
//             voter,
//             proposal: &mut proposal,
//             vote_marker_registry: &mut vote_registry,
//         };

//         // Vote yes with 100 tokens
//         ctx.vote(true, 100).unwrap();
//         assert_eq!(proposal.yes_votes, 100);
//         assert_eq!(proposal.no_votes, 0);

//         // Vote no with 50 tokens (new voter)
//         let voter2 = Pubkey::new_unique();
//         let mut ctx2 = MockVoteCtx {
//             voter: voter2,
//             proposal: &mut proposal,
//             vote_marker_registry: &mut vote_registry,
//         };
//         ctx2.vote(false, 50).unwrap();
//         assert_eq!(proposal.yes_votes, 100);
//         assert_eq!(proposal.no_votes, 50);
//     }

//     #[test]
//     fn test_double_vote_rejected() {
//         let artist = Pubkey::new_unique();
//         let creator = Pubkey::new_unique();
//         let mut proposal = Proposal {
//             artist,
//             creator,
//             title: "Double Vote".to_string(),
//             description: "Desc".to_string(),
//             number_of_tokens: 1000,
//             yes_votes: 0,
//             no_votes: 0,
//             start_date: 1_700_000_000,
//             end_date: 1_700_000_100,
//             status: 0,
//             milestone_reached: false,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 255,
//         };

//         let voter = Pubkey::new_unique();
//         let mut vote_registry = HashSet::new();
//         let mut ctx = MockVoteCtx {
//             voter,
//             proposal: &mut proposal,
//             vote_marker_registry: &mut vote_registry,
//         };

//         ctx.vote(true, 100).unwrap();
//         let res = ctx.vote(true, 50); // same voter tries again
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err(), ErrorCode::AlreadyVoted);
//     }

//     #[test]
//     fn test_vote_after_end_fails() {
//         let artist = Pubkey::new_unique();
//         let creator = Pubkey::new_unique();
//         let mut proposal = Proposal {
//             artist,
//             creator,
//             title: "Ended Vote".to_string(),
//             description: "Desc".to_string(),
//             number_of_tokens: 1000,
//             yes_votes: 0,
//             no_votes: 0,
//             start_date: 1_700_000_000,
//             end_date: 1_699_999_000, // already ended
//             status: 0,
//             milestone_reached: false,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 255,
//         };

//         let voter = Pubkey::new_unique();
//         let mut vote_registry = HashSet::new();
//         let mut ctx = MockVoteCtx {
//             voter,
//             proposal: &mut proposal,
//             vote_marker_registry: &mut vote_registry,
//         };

//         let res = ctx.vote(true, 100);
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err(), ErrorCode::VotingEnded);
//     }

//     #[test]
//     fn test_early_finalize() {
//         let artist = Pubkey::new_unique();
//         let creator = Pubkey::new_unique();
//         let mut proposal = Proposal {
//             artist,
//             creator,
//             title: "Milestone".to_string(),
//             description: "Desc".to_string(),
//             number_of_tokens: 100,
//             yes_votes: 0,
//             no_votes: 0,
//             start_date: 1_700_000_000,
//             end_date: 1_700_000_100,
//             status: 0,
//             milestone_reached: true,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 255,
//         };

//         let voter = Pubkey::new_unique();
//         let mut vote_registry = HashSet::new();
//         let mut ctx = MockVoteCtx {
//             voter,
//             proposal: &mut proposal,
//             vote_marker_registry: &mut vote_registry,
//         };

//         // Voting more than half triggers early finalize
//         ctx.vote(true, 60).unwrap();
//         assert_eq!(proposal.status, 1);
//         assert_eq!(proposal.end_date, MockClock::get().unix_timestamp);
//     }

//     // --------------------------------
//     // Mock TokenAccount for testing
//     // --------------------------------
//     #[derive(Debug, Clone, PartialEq)]
//     struct MockTokenAccount {
//         pub owner: Pubkey,
//         pub mint: Pubkey,
//         pub amount: u64,
//     }

//     impl MockTokenAccount {
//         fn new(owner: Pubkey, mint: Pubkey, amount: u64) -> Self {
//             Self {
//                 owner,
//                 mint,
//                 amount,
//             }
//         }
//     }

//     // --------------------------------
//     // Mock Context for buy handler
//     // --------------------------------
//     struct MockBuyCtx<'a> {
//         pub buyer: Pubkey,
//         pub proposal: &'a mut Proposal,
//         pub artist_vault: &'a mut MockTokenAccount,
//         pub buyer_token_account: &'a mut MockTokenAccount,
//         pub buyer_usdc_account: &'a mut MockTokenAccount,
//         pub usdc_vault: &'a mut MockTokenAccount,
//     }

//     impl<'a> MockBuyCtx<'a> {
//         fn buy(
//             &mut self,
//             amount_usdc: u64,
//             artist_tokens_price: u64,
//             is_campaign_purchase: bool,
//         ) -> Result<u64, ErrorCode> {
//             // Step 1: Calculate tokens bought
//             let tokens_bought = amount_usdc
//                 .checked_div(artist_tokens_price)
//                 .ok_or(ErrorCode::Overflow)?;

//             // Step 2: Simulate USDC transfer
//             if self.buyer_usdc_account.amount < amount_usdc {
//                 return Err(ErrorCode::InsufficientFunds);
//             }
//             self.buyer_usdc_account.amount -= amount_usdc;
//             self.usdc_vault.amount = self
//                 .usdc_vault
//                 .amount
//                 .checked_add(amount_usdc)
//                 .ok_or(ErrorCode::Overflow)?;

//             // Step 3: Simulate artist token transfer
//             if self.artist_vault.amount < tokens_bought {
//                 return Err(ErrorCode::InsufficientVaultBalance);
//             }
//             self.artist_vault.amount -= tokens_bought;
//             self.buyer_token_account.amount = self
//                 .buyer_token_account
//                 .amount
//                 .checked_add(tokens_bought)
//                 .ok_or(ErrorCode::Overflow)?;

//             // Step 4: Update proposal
//             self.proposal.artist_tokens_sold = self
//                 .proposal
//                 .artist_tokens_sold
//                 .checked_add(tokens_bought)
//                 .ok_or(ErrorCode::Overflow)?;
//             self.proposal.usdc_collected = self
//                 .proposal
//                 .usdc_collected
//                 .checked_add(amount_usdc)
//                 .ok_or(ErrorCode::Overflow)?;

//             if is_campaign_purchase {
//                 self.proposal.early_access = true;
//             }

//             Ok(tokens_bought)
//         }
//     }

//     // --------------------------------
//     // Helper: proposal factory
//     // --------------------------------
//     fn mock_proposal() -> Proposal {
//         Proposal {
//             artist: Pubkey::new_unique(),
//             creator: Pubkey::new_unique(),
//             title: "Buy Test".to_string(),
//             description: "Desc".to_string(),
//             number_of_tokens: 1_000_000,
//             yes_votes: 0,
//             no_votes: 0,
//             start_date: 1_700_000_000,
//             end_date: 1_700_000_999,
//             status: 0,
//             milestone_reached: false,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 255,
//         }
//     }

//     // --------------------------------
//     // TEST CASES
//     // --------------------------------

//     #[test]
//     fn test_buy_tokens_success() {
//         let buyer = Pubkey::new_unique();
//         let artist_mint = Pubkey::new_unique();
//         let usdc_mint = Pubkey::new_unique();

//         let mut proposal = mock_proposal();

//         let mut artist_vault = MockTokenAccount::new(Pubkey::new_unique(), artist_mint, 10_000);
//         let mut buyer_tokens = MockTokenAccount::new(buyer, artist_mint, 0);
//         let mut buyer_usdc = MockTokenAccount::new(buyer, usdc_mint, 1_000);
//         let mut usdc_vault = MockTokenAccount::new(Pubkey::new_unique(), usdc_mint, 0);

//         let mut ctx = MockBuyCtx {
//             buyer,
//             proposal: &mut proposal,
//             artist_vault: &mut artist_vault,
//             buyer_token_account: &mut buyer_tokens,
//             buyer_usdc_account: &mut buyer_usdc,
//             usdc_vault: &mut usdc_vault,
//         };

//         let res = ctx.buy(200, 2, false).unwrap(); // 200 USDC, 2 USDC/token
//         assert_eq!(res, 100); // tokens bought
//         assert_eq!(buyer_usdc.amount, 800);
//         assert_eq!(usdc_vault.amount, 200);
//         assert_eq!(buyer_tokens.amount, 100);
//         assert_eq!(artist_vault.amount, 9_900);
//         assert_eq!(proposal.artist_tokens_sold, 100);
//         assert_eq!(proposal.usdc_collected, 200);
//         assert!(!proposal.early_access);
//     }

//     #[test]
//     fn test_buy_with_campaign_flag() {
//         let mut proposal = mock_proposal();
//         let buyer = Pubkey::new_unique();
//         let mint = Pubkey::new_unique();

//         let mut artist_vault = MockTokenAccount::new(Pubkey::new_unique(), mint, 1_000);
//         let mut buyer_tokens = MockTokenAccount::new(buyer, mint, 0);
//         let mut buyer_usdc = MockTokenAccount::new(buyer, mint, 500);
//         let mut usdc_vault = MockTokenAccount::new(Pubkey::new_unique(), mint, 0);

//         let mut ctx = MockBuyCtx {
//             buyer,
//             proposal: &mut proposal,
//             artist_vault: &mut artist_vault,
//             buyer_token_account: &mut buyer_tokens,
//             buyer_usdc_account: &mut buyer_usdc,
//             usdc_vault: &mut usdc_vault,
//         };

//         ctx.buy(100, 2, true).unwrap();
//         assert!(proposal.early_access);
//     }

//     #[test]
//     fn test_buy_insufficient_usdc_fails() {
//         let mut proposal = mock_proposal();
//         let buyer = Pubkey::new_unique();
//         let mint = Pubkey::new_unique();

//         let mut artist_vault = MockTokenAccount::new(Pubkey::new_unique(), mint, 1000);
//         let mut buyer_tokens = MockTokenAccount::new(buyer, mint, 0);
//         let mut buyer_usdc = MockTokenAccount::new(buyer, mint, 50); // only 50 USDC
//         let mut usdc_vault = MockTokenAccount::new(Pubkey::new_unique(), mint, 0);

//         let mut ctx = MockBuyCtx {
//             buyer,
//             proposal: &mut proposal,
//             artist_vault: &mut artist_vault,
//             buyer_token_account: &mut buyer_tokens,
//             buyer_usdc_account: &mut buyer_usdc,
//             usdc_vault: &mut usdc_vault,
//         };

//         let res = ctx.buy(100, 2, false);
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err(), ErrorCode::InsufficientFunds);
//     }

//     #[test]
//     fn test_buy_overflow_protection() {
//         let mut proposal = mock_proposal();
//         let buyer = Pubkey::new_unique();
//         let mint = Pubkey::new_unique();

//         let mut artist_vault = MockTokenAccount::new(Pubkey::new_unique(), mint, u64::MAX);
//         let mut buyer_tokens = MockTokenAccount::new(buyer, mint, u64::MAX);
//         let mut buyer_usdc = MockTokenAccount::new(buyer, mint, u64::MAX);
//         let mut usdc_vault = MockTokenAccount::new(Pubkey::new_unique(), mint, u64::MAX);

//         let mut ctx = MockBuyCtx {
//             buyer,
//             proposal: &mut proposal,
//             artist_vault: &mut artist_vault,
//             buyer_token_account: &mut buyer_tokens,
//             buyer_usdc_account: &mut buyer_usdc,
//             usdc_vault: &mut usdc_vault,
//         };

//         let res = ctx.buy(1, 1, false);
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err(), ErrorCode::Overflow);
//     }

//     #[test]
//     fn test_release_tokens() {
//         // Purpose: Ensure artist can claim USDC and tokens after proposal ends
//         let artist = Pubkey::new_unique();
//         let creator = Pubkey::new_unique();
//         let mut proposal = Proposal {
//             artist,
//             creator,
//             title: "Release Test".to_string(),
//             description: "Desc".to_string(),
//             number_of_tokens: 1000,
//             yes_votes: 500,
//             no_votes: 200,
//             start_date: 1_700_000_000,
//             end_date: 1_700_000_050,
//             status: 1, // finalized
//             milestone_reached: true,
//             early_access: false,
//             usdc_collected: 300,
//             artist_tokens_sold: 150,
//             bump: 255,
//         };

//         let mut artist_token_vault =
//             MockTokenAccount::new(Pubkey::new_unique(), Pubkey::new_unique(), 850); // remaining unsold tokens
//         let mut artist_usdc_account = MockTokenAccount::new(artist, Pubkey::new_unique(), 0); // to receive USDC
//         let mut artist_token_account = MockTokenAccount::new(artist, artist_token_vault.mint, 0); // to receive vested tokens

//         struct MockReleaseCtx<'a> {
//             pub proposal: &'a mut Proposal,
//             pub artist_token_vault: &'a mut MockTokenAccount,
//             pub artist_token_account: &'a mut MockTokenAccount,
//             pub artist_usdc_account: &'a mut MockTokenAccount,
//         }

//         impl<'a> MockReleaseCtx<'a> {
//             fn release(&mut self) -> Result<(), ErrorCode> {
//                 if self.proposal.status != 1 {
//                     return Err(ErrorCode::ProposalNotFinalized);
//                 }

//                 // Transfer USDC
//                 self.artist_usdc_account.amount = self
//                     .artist_usdc_account
//                     .amount
//                     .checked_add(self.proposal.usdc_collected)
//                     .ok_or(ErrorCode::Overflow)?;

//                 // Transfer remaining tokens
//                 let tokens_to_transfer = self.artist_token_vault.amount;
//                 self.artist_token_account.amount = self
//                     .artist_token_account
//                     .amount
//                     .checked_add(tokens_to_transfer)
//                     .ok_or(ErrorCode::Overflow)?;
//                 self.artist_token_vault.amount = 0;

//                 // Reset proposal USDC and tokens sold (optional)
//                 self.proposal.usdc_collected = 0;
//                 self.proposal.artist_tokens_sold = 0;

//                 Ok(())
//             }
//         }

//         let mut ctx = MockReleaseCtx {
//             proposal: &mut proposal,
//             artist_token_vault: &mut artist_token_vault,
//             artist_token_account: &mut artist_token_account,
//             artist_usdc_account: &mut artist_usdc_account,
//         };

//         ctx.release().unwrap();

//         // Assertions
//         assert_eq!(artist_usdc_account.amount, 300); // received collected USDC
//         assert_eq!(artist_token_account.amount, 850); // received remaining tokens
//         assert_eq!(artist_token_vault.amount, 0); // vault emptied
//         assert_eq!(proposal.usdc_collected, 0); // reset
//         assert_eq!(proposal.artist_tokens_sold, 0); // reset
//     }

//     /// Mock Proposal Account Context
//     struct MockFinalizeCtx<'a> {
//         pub caller: Pubkey,
//         pub proposal: &'a mut Proposal,
//         pub current_time: i64,
//     }

//     impl<'a> MockFinalizeCtx<'a> {
//         fn finalize(&mut self) -> Result<(), ErrorCode> {
//             // Step 1: Ensure voting period is over or milestone reached
//             if !(self.current_time >= self.proposal.end_date || self.proposal.milestone_reached) {
//                 return Err(ErrorCode::VotingStillActive);
//             }

//             let total_votes = self.proposal.yes_votes + self.proposal.no_votes;

//             // Step 2: Reject low participation only if milestone not reached
//             if !self.proposal.milestone_reached {
//                 let quorum = self.proposal.number_of_tokens / 10;
//                 if total_votes < quorum {
//                     self.proposal.status = 2; // Rejected due to low participation
//                     return Ok(());
//                 }
//             }

//             // Step 3: Determine final outcome based on majority
//             self.proposal.status = if self.proposal.yes_votes * 100 / total_votes >= 51 {
//                 1 // Approved
//             } else {
//                 2 // Rejected
//             };

//             Ok(())
//         }
//     }

//     #[test]
//     fn test_finalize_approved() {
//         let artist = Pubkey::new_unique();
//         let creator = Pubkey::new_unique();

//         // Set number_of_tokens = 1000 → quorum = 100
//         let mut proposal = Proposal {
//             artist,
//             creator,
//             title: "Approved Proposal".to_string(),
//             description: "Desc".to_string(),
//             number_of_tokens: 1000,
//             yes_votes: 600, // >=51% of total votes
//             no_votes: 200,  // total_votes = 800 > quorum
//             start_date: 1_700_000_000,
//             end_date: 1_700_000_050,
//             status: 0,
//             milestone_reached: false,
//             early_access: false,
//             usdc_collected: 0,
//             artist_tokens_sold: 0,
//             bump: 255,
//         };

//         let mut ctx = MockFinalizeCtx {
//             caller: Pubkey::new_unique(),
//             proposal: &mut proposal,
//             current_time: 1_700_000_100, // after end_date
//         };

//         let res = ctx.finalize();
//         assert!(res.is_ok());
//         assert_eq!(proposal.status, 1); // Approved
//     }

//     #[test]
//     fn test_finalize_rejected_majority_no() {
//         let mut proposal = mock_proposal();
//         proposal.yes_votes = 400;
//         proposal.no_votes = 600;
//         proposal.end_date = 1_700_000_000;

//         let mut ctx = MockFinalizeCtx {
//             caller: Pubkey::new_unique(),
//             proposal: &mut proposal,
//             current_time: 1_700_000_100,
//         };

//         ctx.finalize().unwrap();
//         assert_eq!(proposal.status, 2); // Rejected
//     }

//     #[test]
//     fn test_finalize_low_quorum() {
//         let mut proposal = mock_proposal();
//         proposal.yes_votes = 5;
//         proposal.no_votes = 4; // total 9 < 10% of 1000 tokens
//         proposal.end_date = 1_700_000_000;

//         let mut ctx = MockFinalizeCtx {
//             caller: Pubkey::new_unique(),
//             proposal: &mut proposal,
//             current_time: 1_700_000_100,
//         };

//         ctx.finalize().unwrap();
//         assert_eq!(proposal.status, 2); // Rejected due to low participation
//     }

//     #[test]
//     fn test_finalize_voting_still_active() {
//         let mut proposal = mock_proposal();
//         proposal.end_date = 1_700_000_200; // future
//         proposal.milestone_reached = false;

//         let mut ctx = MockFinalizeCtx {
//             caller: Pubkey::new_unique(),
//             proposal: &mut proposal,
//             current_time: 1_700_000_100,
//         };

//         let res = ctx.finalize();
//         assert!(res.is_err());
//         assert_eq!(res.unwrap_err(), ErrorCode::VotingStillActive);
//     }

//     #[test]
//     fn test_finalize_milestone_override() {
//         let mut proposal = mock_proposal();
//         proposal.end_date = 1_700_000_200; // future
//         proposal.milestone_reached = true;
//         proposal.yes_votes = 600;
//         proposal.no_votes = 200;

//         let mut ctx = MockFinalizeCtx {
//             caller: Pubkey::new_unique(),
//             proposal: &mut proposal,
//             current_time: 1_700_000_100, // before end_date
//         };

//         ctx.finalize().unwrap();
//         assert_eq!(proposal.status, 1); // Approved due to milestone
//     }
// }
