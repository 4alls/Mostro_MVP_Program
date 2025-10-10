# Mostro Smart Contracts

This repository contains the Solana smart contracts (programs) for the Mostro platform — a decentralized platform for artists and their communities. The contracts handle **token creation, DAO governance, and bonding curve mechanics** to allow artists to launch tokens and interact with their fans.

---

## Overview

Mostro allows:

- Artists to **launch their own tokens** (using Token2022) with a fixed supply.
- A **bonding curve mechanism** to manage token sales and liquidity.
- A **DAO-style governance** where token holders vote on artist proposals.
- Controlled token release to artists upon successful proposal approval.

The platform ensures:

- Proper allocation of tokens:
  - **Artist:** 10%
  - **Mostro platform/admin:** 3%
  - **Public via bonding curve:** 87%
- Voting fairness by counting token-based votes per address (considerations for splitting tokens across addresses are being addressed in future updates).

---

## Repository Structure

- `programs/` – Rust-based Solana programs (Anchor framework)
- `scripts/` – TypeScript scripts to interact with programs (e.g., `create_config.ts`)
- `tests/` – TypeScript tests for program logic using Anchor and Chai
- `Anchor.toml` – Anchor project configuration

---

## Features / Smart Contract Responsibilities

### Global Configuration

- Holds platform-wide parameters (percentages, admin wallet, etc.)
- Stored in a **program-derived address (PDA)**.

### Artist Token Management

- Creates an **artist token** with initial distribution.
- Initializes the **bonding curve** for public sale.

### Bonding Curve Logic

- Manages buying and selling of tokens with **exponential pricing formula**.
- Ensures correct allocation and SOL transfers.

### Proposals & Voting

- Artists submit proposals specifying token amount to sell and purpose.
- Token holders vote **yes/no**.
- If approved (>50% yes), tokens are released to the artist.

---

## Tech Stack

- **Solana** – blockchain platform
- **Anchor** – Solana framework for program development
- **TypeScript** – scripting and testing
- **@coral-xyz/anchor** – TypeScript SDK for interacting with programs
- **Chai** – assertion library for tests

---

## Getting Started

### Install dependencies

```bash
npm install
```

### Build the programs

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
