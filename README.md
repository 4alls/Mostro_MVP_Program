# Artist Proposal & Token Management Program

A Solana/Anchor-based program for managing artists, proposals, voting, and token distributions. Tokens are minted via **Pump.fun** and programmatically allocated to artists’ vaults, ensuring secure governance and automated fund release.

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Accounts & PDAs](#accounts--pdas)
4. [Program Flow](#program-flow)
5. [Instructions](#instructions)
6. [Vault & Token Management](#vault--token-management)
7. [Voting & Proposal Mechanics](#voting--proposal-mechanics)
8. [Deployment](#deployment)
9. [Security & Best Practices](#security--best-practices)
10. [Diagram](#diagram)
11. [References](#references)

---

## Overview

This program provides:

- Artist management: Create artist accounts with metadata and custom allocation percentages.
- Proposal management: Artists can submit proposals for community voting.
- Voting: Token-based voting with SPL token balances determining voting power.
- Token release: Upon proposal approval, tokens are automatically released from program-controlled vaults to artists.

Tokens are minted via Pump.fun, ensuring full programmatic control and seamless integration with vaults, proposals, and voting logic.

---

## Architecture

- **Global Config (Config)**: Singleton account storing default percentages for artist and platform, and admin wallet.
- **Artist (Artist)**: PDA representing an artist, including wallet, metadata URI, vault, and allocation percentages.
- **Vault (ArtistVault)**: Program-controlled token account holding the artist's tokens. Tokens are released upon proposal approval.
- **Proposal (Proposal)**: PDA storing proposal info: title, token amount, start/end timestamps, votes, and status.
- **Vote (Vote)**: PDA representing a single voter’s choice and voting power per proposal.

---

## Accounts & PDAs

| Account     | Seeds                                            | Description                                               |
| ----------- | ------------------------------------------------ | --------------------------------------------------------- |
| Config      | `[b"global_config"]`                             | Stores global percentages & admin wallet                  |
| Artist      | `[ARTIST_SEED_PREFIX, artist_wallet]`            | Stores artist-specific data and percentages               |
| ArtistVault | `[ARTIST_VAULT_SEED_PREFIX, artist_wallet]`      | Holds tokens for proposals, controlled by PDA             |
| Proposal    | `[b"artist_proposal", artist_name, proposal_id]` | Stores proposal info (title, token amount, votes, status) |
| Vote        | `[b"vote", proposal.key, voter.key]`             | Tracks individual votes per proposal                      |

---

## Program Flow

1. Admin creates global config and sets default percentages.
2. Admin creates artist; artist account and vault are initialized with default or custom percentages.
3. Artist submits proposal; stored in a Proposal PDA with token allocation.
4. Voters vote; voting power determined by SPL token balance.
5. Finalize proposal after voting period ends, checking quorum and approval threshold.
6. Release tokens to artist; approved proposals release tokens from vaults to artist wallets.

---

## Instructions

### 1. Create Config

**Purpose:** Initialize global configuration with default percentages and admin wallet.  
**Usage Example:**
create_config_handler(context, percentage_artist, percentage_mostro, pump_fun_service_wallet)

**Notes:**

- Percentages must sum ≤ 100.

---

### 2. Create Artist

**Purpose:** Initialize a new artist account and associated vault.  
**Usage Example:**
create_artist_handler(context, metadata_uri, optional_percentage_artist, optional_percentage_mostro)

**Notes:**

- Only the admin can perform this action.
- Percentages default to global config if not provided.

---

### 3. Create Proposal

**Purpose:** Submit a new proposal for voting.  
**Usage Example:**
create_proposal_handler(context, artist_name, proposal_id, title, number_of_tokens)

**Notes:**

- Only the artist authority can create a proposal.
- Proposal duration defaults to 1 week.

---

### 4. Vote Proposal

**Purpose:** Cast a vote on a proposal.  
**Usage Example:**
vote_proposal_handler(context, artist_name, proposal_id, vote_choice)

**Notes:**

- Voting power equals the SPL token balance in the voter’s account.
- Prevents double voting and checks that voting period is active.

---

### 5. Finalize Proposal

**Purpose:** Finalize voting results after the period ends.  
**Usage Example:**
finalize_proposal_handler(context, artist_name, proposal_id)

**Notes:**

- Quorum: at least 10% of total tokens must participate.
- Approval threshold: at least 51% yes votes for approval.

---

### 6. Release Tokens to Artist

**Purpose:** Transfer approved proposal tokens from vault to artist wallet.  
**Usage Example:**
release_tokens_to_artist_handler(context)

**Notes:**

- Only approved proposals (status = approved) can execute.
- Uses program-controlled PDA authority for secure transfer.

---

## Vault & Token Management

- Vaults are program-owned PDAs `[ARTIST_VAULT_SEED_PREFIX, artist_wallet]`.
- PDA authority signs transactions to release tokens securely.
- Tokens minted via Pump.fun are deposited into vaults upon artist creation or as needed.
- Only program PDAs can release tokens to maintain security.

---

## Voting & Proposal Mechanics

- Default voting period: 1 week.
- Voting power = number of tokens in voter’s SPL token account.
- Quorum requirement: 10% of total tokens.
- Approval threshold: 51% yes votes.
- Individual votes tracked per Vote PDA.

---

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
[MostroMVP_Architecture.pdf](https://github.com/user-attachments/files/23205958/MostroMVP_Architecture.pdf)


