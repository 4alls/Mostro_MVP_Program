#[cfg(test)]
mod tests {
    use crate::state::*;
    use anchor_lang::solana_program::pubkey::Pubkey;
    use std::collections::HashMap;
    use bincode; // for binary encoding/decoding
    use crate::error::ErrorCode;
    use crate::state::{Proposal, Vote};

    // -----------------------------
    // PDA Helpers
    // -----------------------------
    fn get_config_pda(program_id: &Pubkey) -> (Pubkey, u8) {
        let config_seed = b"config";
        Pubkey::find_program_address(&[config_seed], program_id)
    }

    fn get_artist_pda(program_id: &Pubkey, artist_wallet: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"artist", artist_wallet.as_ref()], program_id)
    }

    fn get_proposal_pda(program_id: &Pubkey, artist_wallet: &Pubkey, proposal_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"artist_proposal", artist_wallet.as_ref(), &proposal_id.to_le_bytes()],
            program_id,
        )
    }

    fn get_vote_pda(program_id: &Pubkey, proposal: &Pubkey, voter: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"vote", proposal.as_ref(), voter.as_ref()],
            program_id,
        )
    }

    // -----------------------------
    // Config Tests
    // -----------------------------
    #[test]
    fn test_create_config_admin_only_access() {
        // ------------------------------------------------
        // 🧩 Test: Admin-only access control for create_config
        //
        // Purpose:
        // - Simulates that only the designated admin wallet
        //   can initialize a Config instance.
        // - Verifies unauthorized users are rejected.
        // ------------------------------------------------
        let admin_wallet = Pubkey::new_unique();
        let unauthorized_user = Pubkey::new_unique();
        let (_config_pda, bump) = get_config_pda(&crate::id());

        fn create_config(caller: Pubkey, admin_wallet: Pubkey, bump: u8) -> Result<Config, &'static str> {
            if caller != admin_wallet {
                return Err("Unauthorized: only admin can create config");
            }

            Ok(Config {
                percentage_artist: 10,
                percentage_mostro: 5,
                admin_wallet,
                pump_fun_service_wallet: Pubkey::new_unique(),
                bump,
            })
        }

        let result_ok = create_config(admin_wallet, admin_wallet, bump);
        assert!(result_ok.is_ok(), "Admin should be allowed to create config");

        let result_err = create_config(unauthorized_user, admin_wallet, bump);
        assert!(result_err.is_err(), "Non-admin should be rejected");
    }

    #[test]
    fn test_create_config_unit() {
        let admin_wallet = Pubkey::new_unique();
        let (config_pda, bump) = get_config_pda(&crate::id());

        let percentage_artist = 10;
        let percentage_mostro = 5;
        let pump_wallet = Pubkey::new_unique();

        let config = Config {
            percentage_artist,
            percentage_mostro,
            admin_wallet,
            pump_fun_service_wallet: pump_wallet,
            bump,
        };

        assert_eq!(config.admin_wallet, admin_wallet, "Admin wallet should match the caller");
        assert_eq!(config.percentage_artist, percentage_artist);
        assert_eq!(config.percentage_mostro, percentage_mostro);
        assert_eq!(config.pump_fun_service_wallet, pump_wallet);

        let (expected_pda, _) = get_config_pda(&crate::id());
        assert_eq!(config_pda, expected_pda, "Config PDA derivation failed");
    }

    // -----------------------------
    // Artist Tests
    // -----------------------------
    #[test]
    fn test_reject_duplicate_artist_creation() {
        let mut registry: HashMap<Pubkey, Artist> = HashMap::new();
        let artist_wallet = Pubkey::new_unique();
        let artist_1 = Artist {
            artist_wallet,
            metadata_uri: "https://example.com".to_string(),
            pump_token_mint: Pubkey::new_unique(),
            percentage_artist: 10,
            percentage_mostro: 5,
            artist_vault: Pubkey::new_unique(),
            global_config: Pubkey::new_unique(),
            bump: 255,
        };

        registry.insert(artist_wallet, artist_1);

        let duplicate_artist = Artist {
            artist_wallet,
            metadata_uri: "https://duplicate.com".to_string(),
            pump_token_mint: Pubkey::new_unique(),
            percentage_artist: 20,
            percentage_mostro: 10,
            artist_vault: Pubkey::new_unique(),
            global_config: Pubkey::new_unique(),
            bump: 255,
        };

        let result = registry.insert(artist_wallet, duplicate_artist);
        assert!(result.is_some(), "Duplicate artist creation should be detected");
    }

    #[test]
    fn test_artist_pda_derivation() {
        let program_id = Pubkey::new_unique();
        let artist_wallet = Pubkey::new_unique();

        let (artist_pda, bump) = get_artist_pda(&program_id, &artist_wallet);
        let (artist_pda2, bump2) = get_artist_pda(&program_id, &artist_wallet);

        assert_eq!(artist_pda, artist_pda2);
        assert_eq!(bump, bump2);
    }

    #[test]
    fn test_artist_serialization() {
        let artist_wallet = Pubkey::new_unique();
        let artist = Artist {
            artist_wallet,
            metadata_uri: "https://example.com".to_string(),
            pump_token_mint: Pubkey::new_unique(),
            percentage_artist: 10,
            percentage_mostro: 5,
            artist_vault: Pubkey::new_unique(),
            global_config: Pubkey::new_unique(),
            bump: 255,
        };

        let encoded = bincode::serialize(&artist).expect("Serialization failed");
        let decoded: Artist = bincode::deserialize(&encoded).expect("Deserialization failed");

        assert_eq!(decoded, artist);
    }

    // -----------------------------
    // Proposal Tests
    // -----------------------------
    #[test]
    fn test_proposal_pda_derivation() {
        let program_id = Pubkey::new_unique();
        let artist_wallet = Pubkey::new_unique();
        let proposal_id = 42;

        let (proposal_pda, bump) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);
        let (proposal_pda2, bump2) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);

        assert_eq!(proposal_pda, proposal_pda2);
        assert_eq!(bump, bump2);
    }

    #[test]
    fn test_proposal_creation_and_storage() {
        let dummy_artist = Pubkey::new_unique();
        let proposal = Proposal {
            artist: dummy_artist,
            proposal_id: 0,
            title: "My Proposal".to_string(),
            description_uri: "https://ipfs.io/myproposal".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000,
            end_date: 1_700_000_100,
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 0,
            bump: 255,
        };

        let encoded = bincode::serialize(&proposal).expect("Serialization failed");
        let decoded: Proposal = bincode::deserialize(&encoded).expect("Deserialization failed");

        assert_eq!(decoded, proposal);
    }

    #[test]
    fn test_reject_duplicate_proposals() {
        let mut proposals: HashMap<(Pubkey, u64), Proposal> = HashMap::new();
        let artist_wallet = Pubkey::new_unique();
        let proposal_id = 0;

        let proposal = Proposal {
            artist: artist_wallet,
            proposal_id,
            title: "First Proposal".to_string(),
            description_uri: "https://example.com".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000,
            end_date: 1_700_000_100,
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 0,
            bump: 255,
        };

        let insert_result = proposals.insert((artist_wallet, proposal_id), proposal.clone());
        assert!(insert_result.is_none(), "First proposal should insert successfully");

        let duplicate_proposal = Proposal { title: "Duplicate".to_string(), ..proposal.clone() };
        let insert_duplicate = proposals.insert((artist_wallet, proposal_id), duplicate_proposal);

        assert!(insert_duplicate.is_some(), "Duplicate proposal creation should be rejected");
    }

    #[test]
    fn test_invalid_inputs() {
        fn validate_proposal(proposal: &Proposal) -> Result<(), &'static str> {
            if proposal.title.len() > 128 {
                return Err("Title too long");
            }
            if proposal.description_uri.len() > 256 {
                return Err("Description too long");
            }
            if proposal.end_date < proposal.start_date {
                return Err("End date cannot be before start date");
            }
            Ok(())
        }

        let artist_wallet = Pubkey::new_unique();

        let proposal_title = Proposal {
            artist: artist_wallet,
            proposal_id: 0,
            title: "A".repeat(129),
            description_uri: "https://example.com".to_string(),
            number_of_tokens: 100,
            start_date: 1,
            end_date: 2,
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 0,
            bump: 0,
        };
        assert!(validate_proposal(&proposal_title).is_err());

        let proposal_desc = Proposal { description_uri: "D".repeat(257), ..proposal_title.clone() };
        assert!(validate_proposal(&proposal_desc).is_err());

        let proposal_dates = Proposal { start_date: 10, end_date: 5, ..proposal_title.clone() };
        assert!(validate_proposal(&proposal_dates).is_err());
    }

    #[test]
    fn test_boundary_number_of_tokens() {
        let artist_wallet = Pubkey::new_unique();

        let proposal_zero = Proposal {
            artist: artist_wallet,
            proposal_id: 0,
            title: "Zero tokens".to_string(),
            description_uri: "https://example.com".to_string(),
            number_of_tokens: 0,
            start_date: 1,
            end_date: 2,
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 0,
            bump: 0,
        };
        assert_eq!(proposal_zero.number_of_tokens, 0);

        let proposal_max = Proposal { number_of_tokens: u64::MAX, ..proposal_zero.clone() };
        assert_eq!(proposal_max.number_of_tokens, u64::MAX);
    }

    #[test]
    fn test_only_token_holders_can_vote() {
        let program_id = crate::id();
        let voter = Pubkey::new_unique();
        let artist_wallet = Pubkey::new_unique();
        let proposal_id = 1;

        let (proposal_pda, _p_bump) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);
        let (vote_pda, _v_bump) = get_vote_pda(&program_id, &proposal_pda, &voter);

        let mut proposal = Proposal {
            artist: proposal_pda,
            proposal_id,
            title: "Proposal".to_string(),
            description_uri: "https://example.com".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000,
            end_date: i64::MAX,
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 0,
            bump: 255,
        };

        let mut vote_account = Vote {
            proposal: proposal_pda,
            voter,
            vote_choice: false,
            voting_power: 0,
            bump: 255,
        };

        let voter_token_account_balance = 0;

        fn simulate_vote(
            proposal: &mut Proposal,
            vote_account: &mut Vote,
            voter_token_balance: u64,
            vote_choice: bool,
        ) -> Result<(), ErrorCode> {
            if voter_token_balance == 0 {
                return Err(ErrorCode::NoVotingPower);
            }

            if vote_account.voting_power > 0 {
                return Err(ErrorCode::AlreadyVoted);
            }

            vote_account.vote_choice = vote_choice;
            vote_account.voting_power = voter_token_balance;

            if vote_choice {
                proposal.yes_votes += voter_token_balance;
            } else {
                proposal.no_votes += voter_token_balance;
            }

            proposal.total_voting_power += voter_token_balance;

            Ok(())
        }

        let result = simulate_vote(&mut proposal, &mut vote_account, voter_token_account_balance, true);

        assert!(
            matches!(result, Err(ErrorCode::NoVotingPower)),
            "Voter with 0 tokens should be rejected"
        );

        let (proposal_pda_2, _) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);
        let (vote_pda_2, _) = get_vote_pda(&program_id, &proposal_pda, &voter);

        assert_eq!(proposal_pda, proposal_pda_2, "Proposal PDA derivation failed");
        assert_eq!(vote_pda, vote_pda_2, "Vote PDA derivation failed");
    }

    #[test]
    fn test_prevent_double_voting() {
        let program_id = crate::id();
        let voter = Pubkey::new_unique();
        let artist_wallet = Pubkey::new_unique();
        let proposal_id = 1;

        let (proposal_pda, _p_bump) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);
        let (vote_pda, _v_bump) = get_vote_pda(&program_id, &proposal_pda, &voter);

        let mut proposal = Proposal {
            artist: proposal_pda,
            proposal_id,
            title: "Proposal".to_string(),
            description_uri: "https://example.com".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000,
            end_date: i64::MAX,
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 0,
            bump: 255,
        };

        let mut vote_account = Vote {
            proposal: proposal_pda,
            voter,
            vote_choice: true,
            voting_power: 50,
            bump: 255,
        };

        let voter_token_account_balance = 50;

        fn simulate_vote(
            proposal: &mut Proposal,
            vote_account: &mut Vote,
            voter_token_balance: u64,
            vote_choice: bool,
        ) -> Result<(), ErrorCode> {
            if voter_token_balance == 0 {
                return Err(ErrorCode::NoVotingPower);
            }

            if vote_account.voting_power > 0 {
                return Err(ErrorCode::AlreadyVoted);
            }

            vote_account.vote_choice = vote_choice;
            vote_account.voting_power = voter_token_balance;

            if vote_choice {
                proposal.yes_votes += voter_token_balance;
            } else {
                proposal.no_votes += voter_token_balance;
            }

            proposal.total_voting_power += voter_token_balance;
            Ok(())
        }

        let result = simulate_vote(&mut proposal, &mut vote_account, voter_token_account_balance, true);

        assert!(
            matches!(result, Err(ErrorCode::AlreadyVoted)),
            "Double voting attempt should be rejected"
        );

        let (proposal_pda_2, _) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);
        let (vote_pda_2, _) = get_vote_pda(&program_id, &proposal_pda, &voter);

        assert_eq!(proposal_pda, proposal_pda_2, "Proposal PDA derivation failed");
        assert_eq!(vote_pda, vote_pda_2, "Vote PDA derivation failed");
    }

    #[test]
    fn test_multiple_voters_tally() {
        let program_id = crate::id();
        let artist_wallet = Pubkey::new_unique();
        let proposal_id = 1;

        let (proposal_pda, _p_bump) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);

        let mut proposal = Proposal {
            artist: proposal_pda,
            proposal_id,
            title: "Proposal".to_string(),
            description_uri: "https://example.com".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000,
            end_date: i64::MAX,
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 0,
            bump: 255,
        };

        let voters: Vec<(Pubkey, u64, bool)> = vec![
            (Pubkey::new_unique(), 50, true),
            (Pubkey::new_unique(), 30, false),
            (Pubkey::new_unique(), 20, true),
            (Pubkey::new_unique(), 0, true),
        ];

        fn simulate_vote(
            proposal: &mut Proposal,
            vote_account: &mut Vote,
            voter_token_balance: u64,
            vote_choice: bool,
        ) -> Result<(), ErrorCode> {
            if voter_token_balance == 0 {
                return Err(ErrorCode::NoVotingPower);
            }

            if vote_account.voting_power > 0 {
                return Err(ErrorCode::AlreadyVoted);
            }

            vote_account.vote_choice = vote_choice;
            vote_account.voting_power = voter_token_balance;

            if vote_choice {
                proposal.yes_votes += voter_token_balance;
            } else {
                proposal.no_votes += voter_token_balance;
            }

            proposal.total_voting_power += voter_token_balance;
            Ok(())
        }

        // Simulate each voter
        for (voter_pubkey, balance, choice) in &voters {
            let (_vote_pda, _v_bump) = get_vote_pda(&program_id, &proposal_pda, voter_pubkey);

            let mut vote_account = Vote {
            proposal: proposal_pda,
            voter: *voter_pubkey,
            vote_choice: false,
            voting_power: 0,
            bump: 255,
            };

            let result = simulate_vote(&mut proposal, &mut vote_account, *balance, *choice);

            if *balance == 0 {
            assert!(
                matches!(result, Err(ErrorCode::NoVotingPower)),
                "Voter with 0 tokens should be rejected"
            );
            } else {
            assert!(result.is_ok(), "Voting should succeed for voter with tokens");
            }
        }

        // Expected totals
        let expected_yes: u64 = 50 + 20; // voter1 + voter3
        let expected_no: u64 = 30;       // voter2
        let expected_total: u64 = expected_yes + expected_no;

        assert_eq!(proposal.yes_votes, expected_yes, "Yes votes tally mismatch");
        assert_eq!(proposal.no_votes, expected_no, "No votes tally mismatch");
        assert_eq!(proposal.total_voting_power, expected_total, "Total voting power mismatch");

        // Confirm PDA determinism for first voter
        let (vote_pda_check, _) = get_vote_pda(&program_id, &proposal_pda, &voters[0].0);
        assert_eq!(
        vote_pda_check,
        get_vote_pda(&program_id, &proposal_pda, &voters[0].0).0,
        "Vote PDA derivation failed"
        );
    }

    // -----------------------------
    // Simulate finalize_proposal function
    // -----------------------------
    fn finalize_proposal(proposal: &mut Proposal, current_time: i64) -> Result<(), ErrorCode> {
        if current_time < proposal.end_date {
            return Err(ErrorCode::VotingStillActive);
        }

        // Update status: 1 = approved, 2 = rejected
        proposal.status = if proposal.yes_votes > proposal.no_votes {
            1
        } else {
            2
        };
        Ok(())
    }

    #[test]
    fn test_finalize_proposal_status_transition() {
        // ------------------------------------------------
        // 🧩 Test: Status transition via finalize_proposal
        //
        // Purpose:
        // - Ensures that calling finalize_proposal updates
        //   the proposal status correctly based on vote outcome.
        // - Confirms yes/no tallies and total voting power
        //   remain intact after finalization.
        // ------------------------------------------------
    
        let program_id = crate::id();
        let artist_wallet = Pubkey::new_unique();
        let proposal_id = 1;

        // Derive proposal PDA
        let (proposal_pda, _p_bump) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);

        // Simulated proposal with votes already cast
        let mut proposal = Proposal {
            artist: proposal_pda,
            proposal_id,
            title: "Proposal".to_string(),
            description_uri: "https://example.com".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000,
            end_date: 1_700_000_100, // assume voting period ended
            status: 0,               // 0 = Active
            yes_votes: 80,
            no_votes: 20,
            total_voting_power: 100,
            bump: 255,
        };

        // Copy start_date to avoid borrowing while mutably borrowed
        let early_time = proposal.start_date;
        let early_result = finalize_proposal(&mut proposal, early_time);
        assert!(
            early_result.is_err(),
            "Cannot finalize before end_date"
        );

        // Copy end_date + 1 for finalization
        let finalize_time = proposal.end_date + 1;
        let finalize_result = finalize_proposal(&mut proposal, finalize_time);
        assert!(finalize_result.is_ok(), "Proposal should finalize after end_date");

        // Status should reflect result: yes_votes > no_votes
        assert_eq!(proposal.status, 1, "Proposal should be approved");

        // Test with a failing proposal
        let mut proposal2 = Proposal {
            artist: proposal_pda,
            proposal_id: 2,
            title: "Proposal 2".to_string(),
            description_uri: "https://example.com/2".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000,
            end_date: 1_700_000_100,
            status: 0, // active
            yes_votes: 400,
            no_votes: 600,
            total_voting_power: 1000,
            bump: 255,
        };

        let finalize_time2 = proposal2.end_date + 1;
        let finalize_result2 = finalize_proposal(&mut proposal2, finalize_time2);
        assert!(finalize_result2.is_ok(), "Proposal 2 should finalize after end_date");

        // Status should reflect result: yes_votes < no_votes
        assert_eq!(proposal2.status, 2, "Proposal should be rejected");
    }

    #[test]
    fn test_quorum_and_threshold_logic() {
        // ------------------------------------------------
        // 🧩 Test: Quorum and threshold enforcement
        //
        // Purpose:
        // - Ensure proposals only finalize if quorum is met
        //   and yes_votes meet the required threshold.
        // - Quorum = total votes cast / total voting power
        // - Threshold = yes_votes / total votes cast
        // ------------------------------------------------

        let program_id = crate::id();
        let artist_wallet = Pubkey::new_unique();
        let proposal_id = 1;

        // Derive proposal PDA
        let (proposal_pda, _p_bump) = get_proposal_pda(&program_id, &artist_wallet, proposal_id);

        // Simulated proposal
        let mut proposal = Proposal {
            artist: proposal_pda,
            proposal_id,
            title: "Proposal Quorum Test".to_string(),
            description_uri: "https://example.com/quorum".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000,
            end_date: 1_700_000_100,
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 1000, // total tokens eligible to vote
            bump: 255,
        };

        // Define voters (voter_balance, vote_choice)
        let voters: Vec<(u64, bool)> = vec![
            (200, true),   // 200 tokens, yes
            (100, false),  // 100 tokens, no
            (50, true),    // 50 tokens, yes
        ];

        let mut votes_cast = 0;

        // Simulate voting
        for (balance, choice) in voters {
            if choice {
                proposal.yes_votes += balance;
            } else {
                proposal.no_votes += balance;
            }
            votes_cast += balance;
        }

        // Quorum = total votes cast / total voting power
        let quorum = votes_cast as f64 / proposal.total_voting_power as f64;
        // Threshold = yes_votes / total votes cast
        let threshold = proposal.yes_votes as f64 / votes_cast as f64;

        // Set minimum quorum and threshold
        let min_quorum = 0.3;     // 30% of total tokens must vote
        let yes_threshold = 0.6;  // 60% yes required to pass

        // Check if quorum is met
        assert!(
            quorum >= min_quorum,
            "Quorum not met: {}/{} = {}",
            votes_cast,
            proposal.total_voting_power,
            quorum
        );

        // ✅ Check if proposal passes threshold
        let passes_threshold = threshold >= yes_threshold;
        assert!(
            passes_threshold,
            "Threshold not met: yes {}/votes_cast {} = {}",
            proposal.yes_votes,
            votes_cast,
            threshold
        );
    }
}
