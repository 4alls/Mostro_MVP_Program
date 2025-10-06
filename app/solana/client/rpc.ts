import BN from "bn.js";
import {
  AnchorProvider,
  IdlAccounts,
  Program,
  web3,
} from "@coral-xyz/anchor";
import { MethodsBuilder } from "@coral-xyz/anchor/dist/cjs/program/namespace/methods";
import { Mostro } from "../../../target/types/mostro";
import idl from "../../../target/idl/mostro.json";
import * as pda from "./pda";



let _program: Program<Mostro>;


export const initializeClient = (
    programId: web3.PublicKey,
    anchorProvider = AnchorProvider.env(),
) => {
    _program = new Program<Mostro>(
        idl as Mostro,
        anchorProvider,
    );


};

export type CreateConfigArgs = {
  feePayer: web3.PublicKey;
  admin: web3.PublicKey;
  percentageBondingCurve: number;
  percentageArtist: number;
  percentageMostro: number;
  numberOfSolToMigrate: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
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
 */
export const createConfigBuilder = (
	args: CreateConfigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<Mostro, never> => {
  const [configPubkey] = pda.deriveConfigPDA(_program.programId);

  return _program
    .methods
    .createConfig(
      args.percentageBondingCurve,
      args.percentageArtist,
      args.percentageMostro,
      new BN(args.numberOfSolToMigrate.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      config: configPubkey,
      admin: args.admin,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
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
 */
export const createConfig = (
	args: CreateConfigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createConfigBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
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
 */
export const createConfigSendAndConfirm = async (
  args: Omit<CreateConfigArgs, "feePayer" | "admin"> & {
    signers: {
      feePayer: web3.Signer,
      admin: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createConfigBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      admin: args.signers.admin.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.admin])
    .rpc();
}

export type CreateArtistArgs = {
  feePayer: web3.PublicKey;
  artistAuthority: web3.PublicKey;
  tokenMint: web3.PublicKey;
  bondingCurveTokenAccount: web3.PublicKey;
  artistTokenAccount: web3.PublicKey;
  mostroTokenAccount: web3.PublicKey;
  mint: web3.PublicKey;
  funding: web3.PublicKey;
  wallet: web3.PublicKey;
  owner: web3.PublicKey;
  name: string;
  description: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
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
 */
export const createArtistBuilder = (
	args: CreateArtistArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<Mostro, never> => {
  const [configPubkey] = pda.deriveConfigPDA(_program.programId);
    const [artistPubkey] = pda.deriveArtistPDA({
        name: args.name,
    }, _program.programId);
    const [bondingCurveVaultPubkey] = pda.deriveVaultPDA({
        name: args.name,
    }, _program.programId);
    const [assocTokenAccountPubkey] = pda.CslSplTokenPDAs.deriveAccountPDA({
        wallet: args.wallet,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        mint: args.mint,
    }, new web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"));

  return _program
    .methods
    .createArtist(
      args.name,
      args.description,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      config: configPubkey,
      artist: artistPubkey,
      artistAuthority: args.artistAuthority,
      tokenMint: args.tokenMint,
      bondingCurveVault: bondingCurveVaultPubkey,
      bondingCurveTokenAccount: args.bondingCurveTokenAccount,
      artistTokenAccount: args.artistTokenAccount,
      mostroTokenAccount: args.mostroTokenAccount,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
      mint: args.mint,
      funding: args.funding,
      assocTokenAccount: assocTokenAccountPubkey,
      wallet: args.wallet,
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
      owner: args.owner,
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
      associatedTokenProgram: new web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
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
 */
export const createArtist = (
	args: CreateArtistArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createArtistBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
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
 */
export const createArtistSendAndConfirm = async (
  args: Omit<CreateArtistArgs, "feePayer" | "artistAuthority" | "tokenMint" | "bondingCurveTokenAccount" | "artistTokenAccount" | "mostroTokenAccount" | "funding" | "owner"> & {
    signers: {
      feePayer: web3.Signer,
      artistAuthority: web3.Signer,
      tokenMint: web3.Signer,
      bondingCurveTokenAccount: web3.Signer,
      artistTokenAccount: web3.Signer,
      mostroTokenAccount: web3.Signer,
      funding: web3.Signer,
      owner: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createArtistBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      artistAuthority: args.signers.artistAuthority.publicKey,
      tokenMint: args.signers.tokenMint.publicKey,
      bondingCurveTokenAccount: args.signers.bondingCurveTokenAccount.publicKey,
      artistTokenAccount: args.signers.artistTokenAccount.publicKey,
      mostroTokenAccount: args.signers.mostroTokenAccount.publicKey,
      funding: args.signers.funding.publicKey,
      owner: args.signers.owner.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.artistAuthority, args.signers.tokenMint, args.signers.bondingCurveTokenAccount, args.signers.artistTokenAccount, args.signers.mostroTokenAccount, args.signers.funding, args.signers.owner])
    .rpc();
}

export type BuyTokenArgs = {
  feePayer: web3.PublicKey;
  buyer: web3.PublicKey;
  buyerTokenAccount: web3.PublicKey;
  bondingCurveTokenAccount: web3.PublicKey;
  tokenMint: web3.PublicKey;
  source: web3.PublicKey;
  mint: web3.PublicKey;
  destination: web3.PublicKey;
  authority: web3.PublicKey;
  name: string;
  solAmount: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
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
 */
export const buyTokenBuilder = (
	args: BuyTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<Mostro, never> => {
  const [configPubkey] = pda.deriveConfigPDA(_program.programId);
    const [artistPubkey] = pda.deriveArtistPDA({
        name: args.name,
    }, _program.programId);
    const [bondingCurveVaultPubkey] = pda.deriveVaultPDA({
        name: args.name,
    }, _program.programId);

  return _program
    .methods
    .buyToken(
      args.name,
      new BN(args.solAmount.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      config: configPubkey,
      artist: artistPubkey,
      buyer: args.buyer,
      buyerTokenAccount: args.buyerTokenAccount,
      bondingCurveVault: bondingCurveVaultPubkey,
      bondingCurveTokenAccount: args.bondingCurveTokenAccount,
      tokenMint: args.tokenMint,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
      source: args.source,
      mint: args.mint,
      destination: args.destination,
      authority: args.authority,
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
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
 */
export const buyToken = (
	args: BuyTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    buyTokenBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
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
 */
export const buyTokenSendAndConfirm = async (
  args: Omit<BuyTokenArgs, "feePayer" | "buyer" | "buyerTokenAccount" | "authority"> & {
    signers: {
      feePayer: web3.Signer,
      buyer: web3.Signer,
      buyerTokenAccount: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return buyTokenBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      buyer: args.signers.buyer.publicKey,
      buyerTokenAccount: args.signers.buyerTokenAccount.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.buyer, args.signers.buyerTokenAccount, args.signers.authority])
    .rpc();
}

export type SellTokenArgs = {
  feePayer: web3.PublicKey;
  seller: web3.PublicKey;
  sellerTokenAccount: web3.PublicKey;
  bondingCurveTokenAccount: web3.PublicKey;
  tokenMint: web3.PublicKey;
  source: web3.PublicKey;
  mint: web3.PublicKey;
  destination: web3.PublicKey;
  authority: web3.PublicKey;
  name: string;
  tokenAmount: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
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
 */
export const sellTokenBuilder = (
	args: SellTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<Mostro, never> => {
  const [configPubkey] = pda.deriveConfigPDA(_program.programId);
    const [artistPubkey] = pda.deriveArtistPDA({
        name: args.name,
    }, _program.programId);
    const [bondingCurveVaultPubkey] = pda.deriveVaultPDA({
        name: args.name,
    }, _program.programId);

  return _program
    .methods
    .sellToken(
      args.name,
      new BN(args.tokenAmount.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      config: configPubkey,
      artist: artistPubkey,
      seller: args.seller,
      sellerTokenAccount: args.sellerTokenAccount,
      bondingCurveVault: bondingCurveVaultPubkey,
      bondingCurveTokenAccount: args.bondingCurveTokenAccount,
      tokenMint: args.tokenMint,
      source: args.source,
      mint: args.mint,
      destination: args.destination,
      authority: args.authority,
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
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
 */
export const sellToken = (
	args: SellTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    sellTokenBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
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
 */
export const sellTokenSendAndConfirm = async (
  args: Omit<SellTokenArgs, "feePayer" | "seller" | "authority"> & {
    signers: {
      feePayer: web3.Signer,
      seller: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return sellTokenBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      seller: args.signers.seller.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.seller, args.signers.authority])
    .rpc();
}

export type CreateProposalArgs = {
  feePayer: web3.PublicKey;
  artistAuthority: web3.PublicKey;
  tokenMint: web3.PublicKey;
  name: string;
  proposalId: bigint;
  title: string;
  numberOfTokens: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
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
 */
export const createProposalBuilder = (
	args: CreateProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<Mostro, never> => {
    const [artistPubkey] = pda.deriveArtistPDA({
        name: args.name,
    }, _program.programId);
    const [proposalPubkey] = pda.deriveProposalPDA({
        name: args.name,
        proposalId: args.proposalId,
    }, _program.programId);

  return _program
    .methods
    .createProposal(
      args.name,
      new BN(args.proposalId.toString()),
      args.title,
      new BN(args.numberOfTokens.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      artist: artistPubkey,
      proposal: proposalPubkey,
      artistAuthority: args.artistAuthority,
      tokenMint: args.tokenMint,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
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
 */
export const createProposal = (
	args: CreateProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createProposalBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
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
 */
export const createProposalSendAndConfirm = async (
  args: Omit<CreateProposalArgs, "feePayer" | "artistAuthority"> & {
    signers: {
      feePayer: web3.Signer,
      artistAuthority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createProposalBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      artistAuthority: args.signers.artistAuthority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.artistAuthority])
    .rpc();
}

export type VoteOnProposalArgs = {
  feePayer: web3.PublicKey;
  voter: web3.PublicKey;
  voterTokenAccount: web3.PublicKey;
  tokenMint: web3.PublicKey;
  name: string;
  proposalId: bigint;
  voteChoice: boolean;
};

/**
 * ### Returns a {@link MethodsBuilder}
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
 */
export const voteOnProposalBuilder = (
	args: VoteOnProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<Mostro, never> => {
    const [proposalPubkey] = pda.deriveProposalPDA({
        name: args.name,
        proposalId: args.proposalId,
    }, _program.programId);
    const [votePubkey] = pda.deriveVotePDA({
        proposal: args.proposal,
        voter: args.voter,
    }, _program.programId);

  return _program
    .methods
    .voteOnProposal(
      args.name,
      new BN(args.proposalId.toString()),
      args.voteChoice,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      proposal: proposalPubkey,
      vote: votePubkey,
      voter: args.voter,
      voterTokenAccount: args.voterTokenAccount,
      tokenMint: args.tokenMint,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
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
 */
export const voteOnProposal = (
	args: VoteOnProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    voteOnProposalBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
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
 */
export const voteOnProposalSendAndConfirm = async (
  args: Omit<VoteOnProposalArgs, "feePayer" | "voter"> & {
    signers: {
      feePayer: web3.Signer,
      voter: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return voteOnProposalBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      voter: args.signers.voter.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.voter])
    .rpc();
}

export type ExecuteProposalArgs = {
  feePayer: web3.PublicKey;
  artistAuthority: web3.PublicKey;
  artistTokenAccount: web3.PublicKey;
  bondingCurveTokenAccount: web3.PublicKey;
  tokenMint: web3.PublicKey;
  source: web3.PublicKey;
  mint: web3.PublicKey;
  destination: web3.PublicKey;
  authority: web3.PublicKey;
  name: string;
  proposalId: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
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
 */
export const executeProposalBuilder = (
	args: ExecuteProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<Mostro, never> => {
  const [configPubkey] = pda.deriveConfigPDA(_program.programId);
    const [artistPubkey] = pda.deriveArtistPDA({
        name: args.name,
    }, _program.programId);
    const [proposalPubkey] = pda.deriveProposalPDA({
        name: args.name,
        proposalId: args.proposalId,
    }, _program.programId);
    const [bondingCurveVaultPubkey] = pda.deriveVaultPDA({
        name: args.name,
    }, _program.programId);

  return _program
    .methods
    .executeProposal(
      args.name,
      new BN(args.proposalId.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      config: configPubkey,
      artist: artistPubkey,
      proposal: proposalPubkey,
      artistAuthority: args.artistAuthority,
      artistTokenAccount: args.artistTokenAccount,
      bondingCurveVault: bondingCurveVaultPubkey,
      bondingCurveTokenAccount: args.bondingCurveTokenAccount,
      tokenMint: args.tokenMint,
      source: args.source,
      mint: args.mint,
      destination: args.destination,
      authority: args.authority,
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
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
 */
export const executeProposal = (
	args: ExecuteProposalArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    executeProposalBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
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
 */
export const executeProposalSendAndConfirm = async (
  args: Omit<ExecuteProposalArgs, "feePayer" | "authority"> & {
    signers: {
      feePayer: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return executeProposalBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.authority])
    .rpc();
}

// Getters

export const getConfig = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<Mostro>["config"]> => _program.account.config.fetch(publicKey, commitment);

export const getArtist = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<Mostro>["artist"]> => _program.account.artist.fetch(publicKey, commitment);

export const getProposal = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<Mostro>["proposal"]> => _program.account.proposal.fetch(publicKey, commitment);

export const getVote = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<Mostro>["vote"]> => _program.account.vote.fetch(publicKey, commitment);
