import {
  Keypair,
  PublicKey,
  type AccountMeta,
  type TransactionInstruction,
  type TransactionSignature,
} from "@solana/web3.js";
import { useCallback, useEffect } from "react";
import { useConnection } from "@solana/wallet-adapter-react";
import * as programClient from "~/solana/client";

// Props interface for the useProgram hook
export interface UseProgramProps {
  // Optional override for the VITE_SOLANA_PROGRAM_ID env var
  programId?: string;
}

// Error structure returned from sendAndConfirmTx if transaction fails
type SendAndConfirmTxError = {
  message: string;
  logs: string[];
  stack: string | undefined;
};

// Result structure returned from sendAndConfirmTx
type SendAndConfirmTxResult = {
  // Signature of successful transaction
  signature?: string;

  // Error details if transaction fails
  error?: SendAndConfirmTxError;
};

// Helper function to send and confirm a transaction, with error handling
const sendAndConfirmTx = async (
  fn: () => Promise<TransactionSignature>,
): Promise<SendAndConfirmTxResult> => {
  try {
    const signature = await fn();
    return {
      signature,
    };
  } catch (e: any) {
    let message = `An unknown error occurred: ${e}`;
    let logs = [];
    let stack = "";

    if ("logs" in e && e.logs instanceof Array) {
      logs = e.logs;
    }

    if ("stack" in e) {
      stack = e.stack;
    }

    if ("message" in e) {
      message = e.message;
    }

    return {
      error: {
        logs,
        stack,
        message,
      },
    };
  }
};

const useProgram = (props?: UseProgramProps | undefined) => {
  const { connection } = useConnection();

  useEffect(() => {
    let prgId = import.meta.env.VITE_SOLANA_PROGRAM_ID as string | undefined;

    if (props?.programId) {
      prgId = props.programId;
    }

    if (!prgId) {
      throw new Error(
        "the program id must be provided either by the useProgram props or the env var VITE_SOLANA_PROGRAM_ID",
      );
    }

    programClient.initializeClient(new PublicKey(prgId));
  }, [props?.programId, connection.rpcEndpoint]);

  /**
   * Initialize platform configuration
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[writable]` config: {@link Config} Platform configuration account
   * 2. `[signer]` admin: {@link PublicKey} Admin wallet
   * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   *
   * Data:
   * - percentage_bonding_curve: {@link number} Percentage for bonding curve (default 87)
   * - percentage_artist: {@link number} Percentage for artist (default 10)
   * - percentage_mostro: {@link number} Percentage for Mostro (default 3)
   * - number_of_sol_to_migrate: {@link BigInt} SOL threshold for migration
   *
   * @returns {@link TransactionInstruction}
   */
  const createConfig = useCallback(programClient.createConfig, [])

  /**
   * Initialize platform configuration
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[writable]` config: {@link Config} Platform configuration account
   * 2. `[signer]` admin: {@link PublicKey} Admin wallet
   * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   *
   * Data:
   * - percentage_bonding_curve: {@link number} Percentage for bonding curve (default 87)
   * - percentage_artist: {@link number} Percentage for artist (default 10)
   * - percentage_mostro: {@link number} Percentage for Mostro (default 3)
   * - number_of_sol_to_migrate: {@link BigInt} SOL threshold for migration
   *
   * @returns {@link SendAndConfirmTxResult}
   */
  const createConfigSendAndConfirm = useCallback(async (
    args: Omit<programClient.CreateConfigArgs, "feePayer" | "admin"> & {
    signers: {
        feePayer: Keypair,
        admin: Keypair,
    }}, 
    remainingAccounts: Array<AccountMeta> = []
  ): Promise<SendAndConfirmTxResult> => sendAndConfirmTx(() => programClient.createConfigSendAndConfirm(args, remainingAccounts)), [])

  /**
   * Create artist profile and Token2022 mint with distribution
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[]` config: {@link Config} Platform configuration
   * 2. `[writable]` artist: {@link Artist} Artist account to create
   * 3. `[signer]` artist_authority: {@link PublicKey} Artist's wallet authority
   * 4. `[writable, signer]` token_mint: {@link Mint} Token2022 mint to create
   * 5. `[]` bonding_curve_vault: {@link PublicKey} Vault authority for bonding curve tokens
   * 6. `[writable, signer]` bonding_curve_token_account: {@link PublicKey} Token account for bonding curve vault
   * 7. `[writable, signer]` artist_token_account: {@link PublicKey} Token account for artist vault
   * 8. `[writable, signer]` mostro_token_account: {@link PublicKey} Token account for Mostro platform
   * 9. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   * 10. `[writable]` mint: {@link Mint} 
   * 11. `[writable, signer]` funding: {@link PublicKey} Funding account (must be a system account)
   * 12. `[writable]` assoc_token_account: {@link PublicKey} Associated token account address to be created
   * 13. `[]` wallet: {@link PublicKey} Wallet address for the new associated token account
   * 14. `[]` token_program: {@link PublicKey} SPL Token program
   * 15. `[signer]` owner: {@link PublicKey} The mint's minting authority.
   * 16. `[]` token_program: {@link PublicKey} Auto-generated, TokenProgram
   * 17. `[]` associated_token_program: {@link PublicKey} Auto-generated, AssociatedTokenProgram
   *
   * Data:
   * - name: {@link string} Artist name
   * - description: {@link string} type
   *
   * @returns {@link TransactionInstruction}
   */
  const createArtist = useCallback(programClient.createArtist, [])

  /**
   * Create artist profile and Token2022 mint with distribution
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[]` config: {@link Config} Platform configuration
   * 2. `[writable]` artist: {@link Artist} Artist account to create
   * 3. `[signer]` artist_authority: {@link PublicKey} Artist's wallet authority
   * 4. `[writable, signer]` token_mint: {@link Mint} Token2022 mint to create
   * 5. `[]` bonding_curve_vault: {@link PublicKey} Vault authority for bonding curve tokens
   * 6. `[writable, signer]` bonding_curve_token_account: {@link PublicKey} Token account for bonding curve vault
   * 7. `[writable, signer]` artist_token_account: {@link PublicKey} Token account for artist vault
   * 8. `[writable, signer]` mostro_token_account: {@link PublicKey} Token account for Mostro platform
   * 9. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   * 10. `[writable]` mint: {@link Mint} 
   * 11. `[writable, signer]` funding: {@link PublicKey} Funding account (must be a system account)
   * 12. `[writable]` assoc_token_account: {@link PublicKey} Associated token account address to be created
   * 13. `[]` wallet: {@link PublicKey} Wallet address for the new associated token account
   * 14. `[]` token_program: {@link PublicKey} SPL Token program
   * 15. `[signer]` owner: {@link PublicKey} The mint's minting authority.
   * 16. `[]` token_program: {@link PublicKey} Auto-generated, TokenProgram
   * 17. `[]` associated_token_program: {@link PublicKey} Auto-generated, AssociatedTokenProgram
   *
   * Data:
   * - name: {@link string} Artist name
   * - description: {@link string} type
   *
   * @returns {@link SendAndConfirmTxResult}
   */
  const createArtistSendAndConfirm = useCallback(async (
    args: Omit<programClient.CreateArtistArgs, "feePayer" | "artistAuthority" | "tokenMint" | "bondingCurveTokenAccount" | "artistTokenAccount" | "mostroTokenAccount" | "funding" | "owner"> & {
    signers: {
        feePayer: Keypair,
        artistAuthority: Keypair,
        tokenMint: Keypair,
        bondingCurveTokenAccount: Keypair,
        artistTokenAccount: Keypair,
        mostroTokenAccount: Keypair,
        funding: Keypair,
        owner: Keypair,
    }}, 
    remainingAccounts: Array<AccountMeta> = []
  ): Promise<SendAndConfirmTxResult> => sendAndConfirmTx(() => programClient.createArtistSendAndConfirm(args, remainingAccounts)), [])

  /**
   * Purchase tokens from bonding curve
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[]` config: {@link Config} Platform configuration
   * 2. `[writable]` artist: {@link Artist} Artist account
   * 3. `[writable, signer]` buyer: {@link PublicKey} Token buyer
   * 4. `[writable, signer]` buyer_token_account: {@link PublicKey} Buyer's token account
   * 5. `[]` bonding_curve_vault: {@link PublicKey} Bonding curve vault authority
   * 6. `[writable]` bonding_curve_token_account: {@link PublicKey} Bonding curve token account
   * 7. `[]` token_mint: {@link Mint} Token mint
   * 8. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   * 9. `[writable]` source: {@link PublicKey} The source account.
   * 10. `[]` mint: {@link Mint} The token mint.
   * 11. `[writable]` destination: {@link PublicKey} The destination account.
   * 12. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
   * 13. `[]` token_program: {@link PublicKey} Auto-generated, TokenProgram
   *
   * Data:
   * - name: {@link string} Artist name
   * - sol_amount: {@link BigInt} SOL amount to spend in lamports
   *
   * @returns {@link TransactionInstruction}
   */
  const buyToken = useCallback(programClient.buyToken, [])

  /**
   * Purchase tokens from bonding curve
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[]` config: {@link Config} Platform configuration
   * 2. `[writable]` artist: {@link Artist} Artist account
   * 3. `[writable, signer]` buyer: {@link PublicKey} Token buyer
   * 4. `[writable, signer]` buyer_token_account: {@link PublicKey} Buyer's token account
   * 5. `[]` bonding_curve_vault: {@link PublicKey} Bonding curve vault authority
   * 6. `[writable]` bonding_curve_token_account: {@link PublicKey} Bonding curve token account
   * 7. `[]` token_mint: {@link Mint} Token mint
   * 8. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   * 9. `[writable]` source: {@link PublicKey} The source account.
   * 10. `[]` mint: {@link Mint} The token mint.
   * 11. `[writable]` destination: {@link PublicKey} The destination account.
   * 12. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
   * 13. `[]` token_program: {@link PublicKey} Auto-generated, TokenProgram
   *
   * Data:
   * - name: {@link string} Artist name
   * - sol_amount: {@link BigInt} SOL amount to spend in lamports
   *
   * @returns {@link SendAndConfirmTxResult}
   */
  const buyTokenSendAndConfirm = useCallback(async (
    args: Omit<programClient.BuyTokenArgs, "feePayer" | "buyer" | "buyerTokenAccount" | "authority"> & {
    signers: {
        feePayer: Keypair,
        buyer: Keypair,
        buyerTokenAccount: Keypair,
        authority: Keypair,
    }}, 
    remainingAccounts: Array<AccountMeta> = []
  ): Promise<SendAndConfirmTxResult> => sendAndConfirmTx(() => programClient.buyTokenSendAndConfirm(args, remainingAccounts)), [])

  /**
   * Sell tokens back to bonding curve
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[]` config: {@link Config} Platform configuration
   * 2. `[writable]` artist: {@link Artist} Artist account
   * 3. `[writable, signer]` seller: {@link PublicKey} Token seller
   * 4. `[writable]` seller_token_account: {@link PublicKey} Seller's token account
   * 5. `[]` bonding_curve_vault: {@link PublicKey} Bonding curve vault authority
   * 6. `[writable]` bonding_curve_token_account: {@link PublicKey} Bonding curve token account
   * 7. `[]` token_mint: {@link Mint} Token mint
   * 8. `[writable]` source: {@link PublicKey} The source account.
   * 9. `[]` mint: {@link Mint} The token mint.
   * 10. `[writable]` destination: {@link PublicKey} The destination account.
   * 11. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
   * 12. `[]` token_program: {@link PublicKey} Auto-generated, TokenProgram
   *
   * Data:
   * - name: {@link string} Artist name
   * - token_amount: {@link BigInt} Token amount to sell
   *
   * @returns {@link TransactionInstruction}
   */
  const sellToken = useCallback(programClient.sellToken, [])

  /**
   * Sell tokens back to bonding curve
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[]` config: {@link Config} Platform configuration
   * 2. `[writable]` artist: {@link Artist} Artist account
   * 3. `[writable, signer]` seller: {@link PublicKey} Token seller
   * 4. `[writable]` seller_token_account: {@link PublicKey} Seller's token account
   * 5. `[]` bonding_curve_vault: {@link PublicKey} Bonding curve vault authority
   * 6. `[writable]` bonding_curve_token_account: {@link PublicKey} Bonding curve token account
   * 7. `[]` token_mint: {@link Mint} Token mint
   * 8. `[writable]` source: {@link PublicKey} The source account.
   * 9. `[]` mint: {@link Mint} The token mint.
   * 10. `[writable]` destination: {@link PublicKey} The destination account.
   * 11. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
   * 12. `[]` token_program: {@link PublicKey} Auto-generated, TokenProgram
   *
   * Data:
   * - name: {@link string} Artist name
   * - token_amount: {@link BigInt} Token amount to sell
   *
   * @returns {@link SendAndConfirmTxResult}
   */
  const sellTokenSendAndConfirm = useCallback(async (
    args: Omit<programClient.SellTokenArgs, "feePayer" | "seller" | "authority"> & {
    signers: {
        feePayer: Keypair,
        seller: Keypair,
        authority: Keypair,
    }}, 
    remainingAccounts: Array<AccountMeta> = []
  ): Promise<SendAndConfirmTxResult> => sendAndConfirmTx(() => programClient.sellTokenSendAndConfirm(args, remainingAccounts)), [])

  /**
   * Artist creates governance proposal
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[writable]` artist: {@link Artist} Artist account
   * 2. `[writable]` proposal: {@link Proposal} Proposal account to create
   * 3. `[signer]` artist_authority: {@link PublicKey} Artist's wallet authority
   * 4. `[]` token_mint: {@link Mint} Token mint for voting power calculation
   * 5. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   *
   * Data:
   * - name: {@link string} Artist name
   * - proposal_id: {@link BigInt} Proposal ID
   * - title: {@link string} Proposal title
   * - number_of_tokens: {@link BigInt} Tokens to sell if approved
   *
   * @returns {@link TransactionInstruction}
   */
  const createProposal = useCallback(programClient.createProposal, [])

  /**
   * Artist creates governance proposal
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[writable]` artist: {@link Artist} Artist account
   * 2. `[writable]` proposal: {@link Proposal} Proposal account to create
   * 3. `[signer]` artist_authority: {@link PublicKey} Artist's wallet authority
   * 4. `[]` token_mint: {@link Mint} Token mint for voting power calculation
   * 5. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   *
   * Data:
   * - name: {@link string} Artist name
   * - proposal_id: {@link BigInt} Proposal ID
   * - title: {@link string} Proposal title
   * - number_of_tokens: {@link BigInt} Tokens to sell if approved
   *
   * @returns {@link SendAndConfirmTxResult}
   */
  const createProposalSendAndConfirm = useCallback(async (
    args: Omit<programClient.CreateProposalArgs, "feePayer" | "artistAuthority"> & {
    signers: {
        feePayer: Keypair,
        artistAuthority: Keypair,
    }}, 
    remainingAccounts: Array<AccountMeta> = []
  ): Promise<SendAndConfirmTxResult> => sendAndConfirmTx(() => programClient.createProposalSendAndConfirm(args, remainingAccounts)), [])

  /**
   * Token holders vote on proposals
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[writable]` proposal: {@link Proposal} Proposal to vote on
   * 2. `[writable]` vote: {@link Vote} Vote account to create
   * 3. `[signer]` voter: {@link PublicKey} Voter's wallet
   * 4. `[]` voter_token_account: {@link PublicKey} Voter's token account
   * 5. `[]` token_mint: {@link Mint} Token mint
   * 6. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   *
   * Data:
   * - name: {@link string} Artist name
   * - proposal_id: {@link BigInt} Proposal ID
   * - vote_choice: {@link boolean} Vote choice (true=yes, false=no)
   *
   * @returns {@link TransactionInstruction}
   */
  const voteOnProposal = useCallback(programClient.voteOnProposal, [])

  /**
   * Token holders vote on proposals
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[writable]` proposal: {@link Proposal} Proposal to vote on
   * 2. `[writable]` vote: {@link Vote} Vote account to create
   * 3. `[signer]` voter: {@link PublicKey} Voter's wallet
   * 4. `[]` voter_token_account: {@link PublicKey} Voter's token account
   * 5. `[]` token_mint: {@link Mint} Token mint
   * 6. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
   *
   * Data:
   * - name: {@link string} Artist name
   * - proposal_id: {@link BigInt} Proposal ID
   * - vote_choice: {@link boolean} Vote choice (true=yes, false=no)
   *
   * @returns {@link SendAndConfirmTxResult}
   */
  const voteOnProposalSendAndConfirm = useCallback(async (
    args: Omit<programClient.VoteOnProposalArgs, "feePayer" | "voter"> & {
    signers: {
        feePayer: Keypair,
        voter: Keypair,
    }}, 
    remainingAccounts: Array<AccountMeta> = []
  ): Promise<SendAndConfirmTxResult> => sendAndConfirmTx(() => programClient.voteOnProposalSendAndConfirm(args, remainingAccounts)), [])

  /**
   * Execute approved proposal (sell tokens, send SOL to artist)
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[]` config: {@link Config} Platform configuration
   * 2. `[writable]` artist: {@link Artist} Artist account
   * 3. `[writable]` proposal: {@link Proposal} Proposal to execute
   * 4. `[writable]` artist_authority: {@link PublicKey} Artist's wallet to receive SOL
   * 5. `[writable]` artist_token_account: {@link PublicKey} Artist's token account
   * 6. `[writable]` bonding_curve_vault: {@link PublicKey} Bonding curve vault authority
   * 7. `[writable]` bonding_curve_token_account: {@link PublicKey} Bonding curve token account
   * 8. `[]` token_mint: {@link Mint} Token mint
   * 9. `[writable]` source: {@link PublicKey} The source account.
   * 10. `[]` mint: {@link Mint} The token mint.
   * 11. `[writable]` destination: {@link PublicKey} The destination account.
   * 12. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
   * 13. `[]` token_program: {@link PublicKey} Auto-generated, TokenProgram
   *
   * Data:
   * - name: {@link string} Artist name
   * - proposal_id: {@link BigInt} Proposal ID
   *
   * @returns {@link TransactionInstruction}
   */
  const executeProposal = useCallback(programClient.executeProposal, [])

  /**
   * Execute approved proposal (sell tokens, send SOL to artist)
   *
   * Accounts:
   * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
   * 1. `[]` config: {@link Config} Platform configuration
   * 2. `[writable]` artist: {@link Artist} Artist account
   * 3. `[writable]` proposal: {@link Proposal} Proposal to execute
   * 4. `[writable]` artist_authority: {@link PublicKey} Artist's wallet to receive SOL
   * 5. `[writable]` artist_token_account: {@link PublicKey} Artist's token account
   * 6. `[writable]` bonding_curve_vault: {@link PublicKey} Bonding curve vault authority
   * 7. `[writable]` bonding_curve_token_account: {@link PublicKey} Bonding curve token account
   * 8. `[]` token_mint: {@link Mint} Token mint
   * 9. `[writable]` source: {@link PublicKey} The source account.
   * 10. `[]` mint: {@link Mint} The token mint.
   * 11. `[writable]` destination: {@link PublicKey} The destination account.
   * 12. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
   * 13. `[]` token_program: {@link PublicKey} Auto-generated, TokenProgram
   *
   * Data:
   * - name: {@link string} Artist name
   * - proposal_id: {@link BigInt} Proposal ID
   *
   * @returns {@link SendAndConfirmTxResult}
   */
  const executeProposalSendAndConfirm = useCallback(async (
    args: Omit<programClient.ExecuteProposalArgs, "feePayer" | "authority"> & {
    signers: {
        feePayer: Keypair,
        authority: Keypair,
    }}, 
    remainingAccounts: Array<AccountMeta> = []
  ): Promise<SendAndConfirmTxResult> => sendAndConfirmTx(() => programClient.executeProposalSendAndConfirm(args, remainingAccounts)), [])


  const getConfig = useCallback(programClient.getConfig, [])
  const getArtist = useCallback(programClient.getArtist, [])
  const getProposal = useCallback(programClient.getProposal, [])
  const getVote = useCallback(programClient.getVote, [])

  const deriveConfig = useCallback(programClient.deriveConfigPDA,[])
  const deriveArtist = useCallback(programClient.deriveArtistPDA,[])
  const deriveVault = useCallback(programClient.deriveVaultPDA,[])
  const deriveProposal = useCallback(programClient.deriveProposalPDA,[])
  const deriveVote = useCallback(programClient.deriveVotePDA,[])
  const deriveAccountFromTokenProgram = useCallback(programClient.TokenProgramPDAs.deriveAccountPDA, [])

  return {
    createConfig,
    createConfigSendAndConfirm,
    createArtist,
    createArtistSendAndConfirm,
    buyToken,
    buyTokenSendAndConfirm,
    sellToken,
    sellTokenSendAndConfirm,
    createProposal,
    createProposalSendAndConfirm,
    voteOnProposal,
    voteOnProposalSendAndConfirm,
    executeProposal,
    executeProposalSendAndConfirm,
    getConfig,
    getArtist,
    getProposal,
    getVote,
    deriveConfig,
    deriveArtist,
    deriveVault,
    deriveProposal,
    deriveVote,
    deriveAccountFromTokenProgram,
  };
};

export { useProgram };