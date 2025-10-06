import {PublicKey} from "@solana/web3.js";
import BN from "bn.js";

export const deriveConfigPDA = (programId: PublicKey): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("config"),
        ],
        programId,
    )
};

export type ArtistSeeds = {
    name: string, 
};

export const deriveArtistPDA = (
    seeds: ArtistSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("artist"),
            Buffer.from(seeds.name, "utf8"),
        ],
        programId,
    )
};

export type VaultSeeds = {
    name: string, 
};

export const deriveVaultPDA = (
    seeds: VaultSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("vault"),
            Buffer.from(seeds.name, "utf8"),
        ],
        programId,
    )
};

export type ProposalSeeds = {
    name: string, 
    proposalId: bigint, 
};

export const deriveProposalPDA = (
    seeds: ProposalSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("artist_proposal"),
            Buffer.from(seeds.name, "utf8"),
            Buffer.from(BigUint64Array.from([seeds.proposalId]).buffer),
        ],
        programId,
    )
};

export type VoteSeeds = {
    proposal: PublicKey, 
    voter: PublicKey, 
};

export const deriveVotePDA = (
    seeds: VoteSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("vote"),
            seeds.proposal.toBuffer(),
            seeds.voter.toBuffer(),
        ],
        programId,
    )
};

export module TokenProgramPDAs {
    export type AccountSeeds = {
        wallet: PublicKey, 
        tokenProgram: PublicKey, 
        mint: PublicKey, 
    };
    
    export const deriveAccountPDA = (
        seeds: AccountSeeds,
        programId: PublicKey
    ): [PublicKey, number] => {
        return PublicKey.findProgramAddressSync(
            [
                seeds.wallet.toBuffer(),
                seeds.tokenProgram.toBuffer(),
                seeds.mint.toBuffer(),
            ],
            programId,
        )
    };
    
}

export module AssociatedTokenProgramPDAs {
    export module TokenProgramPDAs {
        export type AccountSeeds = {
            wallet: PublicKey, 
            tokenProgram: PublicKey, 
            mint: PublicKey, 
        };
        
        export const deriveAccountPDA = (
            seeds: AccountSeeds,
            programId: PublicKey
        ): [PublicKey, number] => {
            return PublicKey.findProgramAddressSync(
                [
                    seeds.wallet.toBuffer(),
                    seeds.tokenProgram.toBuffer(),
                    seeds.mint.toBuffer(),
                ],
                programId,
            )
        };
        
    }
    
}

