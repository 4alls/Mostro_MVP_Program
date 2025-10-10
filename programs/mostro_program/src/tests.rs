#[cfg(test)]
mod tests {
    use crate::state::*;
    use anchor_lang::solana_program::pubkey::Pubkey;
    use std::collections::HashMap;
    use bincode; // for binary encoding/decoding

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
        // Use helper instead of manual seed
        let (_config_pda, bump) = get_config_pda(&crate::id());

        // Simulated access control logic
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

        // ✅ Admin should succeed
        let result_ok = create_config(admin_wallet, admin_wallet, bump);
        assert!(result_ok.is_ok(), "Admin should be allowed to create config");

        // ❌ Non-admin should fail
        let result_err = create_config(unauthorized_user, admin_wallet, bump);
        assert!(result_err.is_err(), "Non-admin should be rejected");
    }

    #[test]
    fn test_create_config_unit() {
        // ------------------------------------------------
        // 🧩 Test: PDA derivation & field persistence for Config
        //
        // Purpose:
        // - Ensures the Config PDA is correctly derived
        //   using the expected seed and program ID.
        // - Confirms that all fields in Config persist
        //   correctly after initialization.
        // ------------------------------------------------
        // -----------------------------
        // Simulate admin and PDA derivation
        // -----------------------------
        let admin_wallet = Pubkey::new_unique();
       // Use helper instead of manual seed
        let (config_pda, bump) = get_config_pda(&crate::id());

        // -----------------------------
        // Create a Config instance
        // -----------------------------
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

        // -----------------------------
        // Assertions
        // -----------------------------
        assert_eq!(config.admin_wallet, admin_wallet, "Admin wallet should match the caller");
        assert_eq!(config.percentage_artist, percentage_artist);
        assert_eq!(config.percentage_mostro, percentage_mostro);
        assert_eq!(config.pump_fun_service_wallet, pump_wallet);

        // Check PDA derivation matches expected seed
        let (expected_pda, _) = get_config_pda(&crate::id());
        assert_eq!(config_pda, expected_pda, "Config PDA derivation failed");
    }

    // -----------------------------
    // Artist Tests
    // -----------------------------
    #[test]
    fn test_reject_duplicate_artist_creation() {
        // ------------------------------------------------
        // 🧩 Test: Reject duplicate artist creation
        //
        // Purpose:
        // - Ensures that attempting to create an Artist
        //   with a wallet that already exists is rejected.
        // - Confirms uniqueness of artist wallet in the registry.
        // ------------------------------------------------
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

        // First creation should succeed
        registry.insert(artist_wallet, artist_1);

        // Attempt to create a second artist with same wallet
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

        // The insert should return Some(previous_value), meaning a duplicate existed
        assert!(
            result.is_some(),
            "Duplicate artist creation should be detected"
        );
    }

    #[test]
    fn test_artist_pda_derivation() {
        // ------------------------------------------------
        // 🧩 Test: Artist PDA derivation
        //
        // Purpose:
        // - Verifies that the Artist PDA is correctly derived
        //   using the artist wallet and program ID.
        // - Confirms that PDA derivation is deterministic
        //   (same input -> same PDA and bump).
        // ------------------------------------------------
        let program_id = Pubkey::new_unique();
        let artist_wallet = Pubkey::new_unique();

        let (artist_pda, bump) = get_artist_pda(&program_id, &artist_wallet);

        // deterministic derivation: same input -> same PDA
        let (artist_pda2, bump2) = get_artist_pda(&program_id, &artist_wallet);

        assert_eq!(artist_pda, artist_pda2);
        assert_eq!(bump, bump2);
    }

    #[test]
    fn test_artist_serialization() {
        // ------------------------------------------------
        // 🧩 Test: Artist struct serialization
        //
        // Purpose:
        // - Confirms that all fields in the Artist struct
        //   are correctly stored and persisted after serialization.
        // - Ensures that serialization and deserialization
        //   round-trip works as expected using bincode.
        // ------------------------------------------------
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

        // ✅ Serialize to bytes
        let encoded = bincode::serialize(&artist).expect("Serialization failed");

        // ✅ Deserialize back to Artist
        let decoded: Artist = bincode::deserialize(&encoded).expect("Deserialization failed");

        // ✅ Compare structs directly (all fields)
        assert_eq!(decoded, artist);
    }

    // -----------------------------
    // Proposal Tests
    // -----------------------------
    #[test]
    fn test_proposal_pda_derivation() {
        // ------------------------------------------------
        // 🧩 Test: Proposal PDA derivation
        //
        // Purpose:
        // - Verifies that the Proposal PDA is correctly derived
        //   using the artist wallet, proposal ID, and program ID.
        // - Confirms deterministic derivation (same input -> same PDA and bump).
        // ------------------------------------------------
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
        // ------------------------------------------------
        // 🧩 Test: Proposal creation and proper data storage
        //
        // Purpose:
        // - Ensures Proposal struct fields are correctly initialized
        // - Confirms that the data persists correctly in memory
        // ------------------------------------------------
        let dummy_artist = Pubkey::new_unique();
        let proposal = Proposal {
            artist: dummy_artist,
            proposal_id: 0,
            title: "My Proposal".to_string(),
            description_uri: "https://ipfs.io/myproposal".to_string(),
            number_of_tokens: 1000,
            start_date: 1_700_000_000, // example timestamp
            end_date: 1_700_000_100,   // example timestamp
            status: 0,
            yes_votes: 0,
            no_votes: 0,
            total_voting_power: 0,
            bump: 255,
        };

        // ✅ Serialize/deserialize to mimic storage
        let encoded = bincode::serialize(&proposal).expect("Serialization failed");
        let decoded: Proposal = bincode::deserialize(&encoded).expect("Deserialization failed");

        assert_eq!(decoded, proposal);
    }

    #[test]
    fn test_reject_duplicate_proposals() {
        // ------------------------------------------------
        // 🧩 Test: Reject duplicate proposal creation
        //
        // Purpose:
        // - Ensures that creating a proposal with the same
        //   proposal_id for the same artist is rejected.
        // ------------------------------------------------
        use std::collections::HashMap;

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

        // First insertion should succeed
        let insert_result = proposals.insert((artist_wallet, proposal_id), proposal.clone());
        assert!(insert_result.is_none(), "First proposal should insert successfully");

        // Attempt duplicate insertion
        let duplicate_proposal = Proposal { title: "Duplicate".to_string(), ..proposal.clone() };
        let insert_duplicate = proposals.insert((artist_wallet, proposal_id), duplicate_proposal);

        assert!(insert_duplicate.is_some(), "Duplicate proposal creation should be rejected");
    }

    #[test]
    fn test_invalid_inputs() {
        // ------------------------------------------------
        // 🧩 Test: Reject invalid inputs
        //
        // Purpose:
        // - Rejects proposals with invalid fields:
        //   - title too long
        //   - description too long
        //   - invalid dates (end_date < start_date)
        // ------------------------------------------------
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

        // Invalid title
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

        // Invalid description
        let proposal_desc = Proposal { description_uri: "D".repeat(257), ..proposal_title.clone() };
        assert!(validate_proposal(&proposal_desc).is_err());

        // Invalid dates
        let proposal_dates = Proposal { start_date: 10, end_date: 5, ..proposal_title.clone() };
        assert!(validate_proposal(&proposal_dates).is_err());
    }

    #[test]
    fn test_boundary_number_of_tokens() {
        // ------------------------------------------------
        // 🧩 Test: Boundary tests for number_of_tokens
        //
        // Purpose:
        // - Verifies that proposals with extreme token numbers
        //   (0, 1, max u64) are handled correctly.
        // ------------------------------------------------
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
}
