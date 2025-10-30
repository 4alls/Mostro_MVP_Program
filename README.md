# Artist Proposal & Token Management Program (Summary)

A Solana/Anchor program for managing artists, proposals, token sales, and voting. Tokens are minted via **Pump.fun** and securely held in program-controlled vaults.

---

## Overview

- **Artist Management:** Create artist accounts with metadata, mint references, and token allocation.
- **Proposals:** Artists can submit proposals for community voting.
- **Voting:** Token-weighted voting using SPL token balances.
- **Token Release:** Approved proposals release tokens from vaults to artist wallets.
- **Early Access / Milestones:** Supports early milestone approval and early-access purchases.

---

## Key Accounts & PDAs

| Account          | Seed / PDA Pattern                                           | Purpose |
|-----------------|-------------------------------------------------------------|---------|
| **Artist**       | `[b"artist", artist_name]`                                  | Stores artist metadata, mint, total tokens |
| **Proposal**     | `[b"proposal", artist.key(), creator.key()]`               | Stores proposal details (title, description, tokens, votes, status) |
| **Vote Marker**  | `[b"vote", proposal.key(), voter.key()]`                   | Prevents double voting |
| **Artist Vault** | `[b"artist_vault", proposal.key()]`                        | Holds artist tokens securely |
| **USDC Vault**   | `[b"usdc_vault", proposal.key()]`                           | Holds collected USDC during token sales |

---

## Program Flow

1. **Create Artist:** Admin initializes artist PDA and vault with metadata and token allocation.  
2. **Create Proposal:** Artist (or creator) submits a proposal; stores title, description, token allocation, milestone flags, and voting period.  
3. **Vote Proposal:** Users vote; voting power = SPL token balance. Vote marker PDA prevents double voting.  
4. **Finalize Proposal:** After end date or milestone reached; checks quorum (10%) and majority (≥51%) to approve/reject.  
5. **Release Tokens:** Admin releases USDC and/or artist tokens from vaults to artist wallets. Handles approved, rejected, or milestone-based early approvals.  
6. **Buy Tokens for Proposal:** Users can purchase artist tokens with USDC. Tokens transferred from artist vault to buyer; USDC collected in program vault.

---

## Voting & Token Rules

- **Default Voting Period:** 10 days (configurable)  
- **Quorum:** ≥10% of total allocated tokens must participate  
- **Approval Threshold:** ≥51% yes votes  
- **Early Milestone Approval:** If milestone reached and majority yes votes, proposal can finalize early  
- **Early Access:** Campaign purchases can flag early-access votes  

---

## Vault & Token Management

- **PDAs:** Only program-controlled PDAs can release tokens.
- **USDC & Tokens:** USDC collected in vaults; artist tokens released to buyers or artists.  
- **Financial Safety:** Resets collected USDC and sold tokens after release to prevent double spending.  
- **Token Pricing:** Program calculates tokens per USDC spent.

---

## Security & Best Practices

- Only admin or artist authority can call critical instructions.  
- PDA seeds and bumps must be verified before actions.  
- Tokens remain in vaults until proposal approval.  
- Vote marker PDAs prevent double voting.  
- Regular audits recommended for token allocation and voting logic.

---

## Getting Started

**Tech Stack:**

- Solana  
- Anchor  
- TypeScript  
- @coral-xyz/anchor SDK  
- Chai (for tests)

**Deployment / Testing:**

```bash
anchor build -- --features anchor
cargo test --features manual
anchor test
npx ts-node scripts/create_config.ts
```

## Getting Started

### Tech Stack

- **Solana** – blockchain platform
- **Anchor** – Solana framework for program development
- **TypeScript** – scripting and testing
- **@coral-xyz/anchor** – TypeScript SDK for interacting with programs
- **Chai** – assertion library for tests

---

### Install dependencies

```bash
npm install
```

## Deployment

**Build program:**

```bash
anchor clean
anchor build -- --features anchor
```

### Run tests

```bash
cargo test --features manual
```

```bash
anchor test
```

### Run scripts (e.g., create config)

```bash
npx ts-node scripts/create_config.ts
```

**Verify accounts:**

- Ensure Config PDA exists.
- Artist vaults and token accounts are initialized correctly.

---

## Security & Best Practices

- Always verify PDA seeds and bumps when interacting with accounts.
- Ensure only admin or artist authority signs critical instructions.
- Tokens must remain in vaults until proposals are approved.
- Conduct regular audits for allocation logic and voting calculations.

---

## Diagram
<img width="1024" height="1024" alt="MostroMVP_Architecture" src="https://github.com/user-attachments/assets/1cb0bddf-26a1-496b-97aa-3918cdf1dfeb" />



