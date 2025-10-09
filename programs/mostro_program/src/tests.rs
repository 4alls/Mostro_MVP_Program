#[cfg(test)]
mod tests {
    use tokio;
    use super::*;
    use crate::state::*;
    use crate::instructions::*;
    use borsh::{BorshSerialize, BorshDeserialize};
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::{system_program, pubkey::Pubkey, instruction::Instruction};
    use anchor_lang::{InstructionData, AccountDeserialize};
    use solana_program_test::{ProgramTest, processor};
    use solana_sdk::{
        account::Account as SolanaAccount,
        signature::Keypair,
        signer::Signer,
        transaction::Transaction,
        instruction::AccountMeta,
    };

    // -----------------------------
    // PDA Helpers
    // -----------------------------
    fn get_artist_pda(program_id: &Pubkey, artist_wallet: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"artist", artist_wallet.as_ref()], program_id)
    }

    fn get_proposal_pda(program_id: &Pubkey, artist_wallet: &Pubkey, proposal_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"artist_proposal", artist_wallet.as_ref(), &proposal_id.to_le_bytes()],
            program_id,
        )
    }

    #[test]
    fn test_create_config_unit() {
        // -----------------------------
        // Simulate admin and PDA derivation
        // -----------------------------
        let admin_wallet = Pubkey::new_unique();
        let config_seed = b"config";
        let (config_pda, bump) = Pubkey::find_program_address(&[config_seed], &crate::id());

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
        let (expected_pda, _) = Pubkey::find_program_address(&[config_seed], &crate::id());
        assert_eq!(config_pda, expected_pda, "Config PDA derivation failed");
    }

    // -----------------------------
    // Unit test: PDA derivation
    // -----------------------------
    #[test]
    fn test_artist_pda_derivation() {
        let program_id = Pubkey::new_unique();
        let artist_wallet = Pubkey::new_unique();

        let (artist_pda, bump) = get_artist_pda(&program_id, &artist_wallet);

        // deterministic derivation: same input -> same PDA
        let (artist_pda2, bump2) = get_artist_pda(&program_id, &artist_wallet);

        assert_eq!(artist_pda, artist_pda2);
        assert_eq!(bump, bump2);
    }

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

    // -----------------------------
    // Unit test: Artist struct serialization
    // -----------------------------
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

        let mut data = vec![0u8; Artist::space()];
        let mut cursor = std::io::Cursor::new(&mut data[..]);
        artist.try_serialize(&mut cursor).expect("Serialization failed");

        // Deserialize and check equality
        let mut data_slice: &[u8] = &data;
        let deserialized = Artist::try_deserialize(&mut data_slice).expect("Deserialization failed");

        assert_eq!(deserialized.artist_wallet, artist.artist_wallet);
        assert_eq!(deserialized.metadata_uri, artist.metadata_uri);
        assert_eq!(deserialized.percentage_artist, artist.percentage_artist);
        assert_eq!(deserialized.percentage_mostro, artist.percentage_mostro);
    }

    // -----------------------------
    // Unit test: Proposal struct creation
    // -----------------------------
    #[test]
    fn test_proposal_creation_logic() {
        let dummy_artist = Pubkey::new_unique();
        
        let proposal = Proposal {
        artist: dummy_artist,
        proposal_id: 0,
        title: "My Proposal".to_string(),
        description_uri: "https://ipfs.io/myproposal".to_string(),
        number_of_tokens: 1000,
        start_date: 0,          // dummy timestamp for testing
        end_date: 0,            // dummy timestamp for testing
        status: 0,              // e.g., 0 = pending
        yes_votes: 0,
        no_votes: 0,
        total_voting_power: 0,
        bump: 255,              // dummy PDA bump
    };

        assert_eq!(proposal.proposal_id, 0);
        assert_eq!(proposal.title, "My Proposal");
        assert_eq!(proposal.number_of_tokens, 1000);
    }
}