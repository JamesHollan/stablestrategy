# Stable Strategy

CoyGSCf4YMxozas3RHfwPt4QA93fLrDE7m3WtmQ9pump

Stable Strategy is an on-chain Solana program designed to convert SOL (or a project’s native token) into a stable Solana-based token and distribute it to the top 250 holders of a given SPL token. The goal is to provide a transparent, rules-based “stability layer” for early supporters of a project.

> **Disclaimer:** This repository is for educational and experimental purposes only. It is **not** audited and **must not** be used in production without a professional security review.

---

## Overview

**Stable Strategy** implements a simple mechanism:

1. A project deploys this program and configures:
   - The SPL token mint of the project.
   - The Solana-based stablecoin mint (e.g. a stable SPL token).
   - The authority allowed to trigger the distribution.
2. The program:
   - Reads the top 250 holders of the project token (off-chain indexing + on-chain verification).
   - Allocates a predefined pool of stable tokens.
   - Distributes the stable tokens proportionally or equally (configurable) to those 250 holders.

This creates a transparent, on-chain “reward layer” for early holders.

---

## Key Features

- **Top 250 holders targeting:**  
  The distribution is limited to the first 250 holders of the configured SPL token.

- **Stable token distribution:**  
  Converts a pool of SOL or project tokens into a stable Solana-based token (via off-chain or external swap) and then distributes that stable token.

- **Configurable distribution logic:**  
  - Equal distribution per address, or  
  - Proportional to token holdings (example logic included).

- **Anchor-based Solana program:**  
  Built using the [Anchor framework](https://www.anchor-lang.com/) for easier development and testing.

---

## Architecture

### On-chain program

The on-chain program (under `programs/stable_strategy`) is responsible for:

- Storing configuration:
  - Project token mint.
  - Stable token mint.
  - Distribution authority.
  - Distribution mode (equal / proportional).
- Verifying recipients:
  - Ensuring that each recipient is a valid holder of the project token.
- Executing the distribution:
  - Transferring stable tokens from a vault account to the recipients.

### Off-chain scripts

The off-chain script (under `scripts/distribute.ts`) is responsible for:

- Fetching the top 250 holders of the project token (via RPC/indexer).
- Preparing the list of recipient accounts.
- Sending the transaction(s) to the on-chain program to perform the distribution.

---

## Repository structure

```text
programs/
  stable_strategy/
    Cargo.toml        # Rust/Anchor program configuration
    src/
      lib.rs          # Main on-chain program logic

scripts/
  distribute.ts       # Off-chain script to trigger distribution

tests/
  stable_strategy.test.ts  # Example tests using Anchor/TypeScript

Anchor.toml           # Anchor workspace configuration
package.json          # Node/TypeScript dependencies
tsconfig.json         # TypeScript configuration
README.md             # Project documentation


<img width="1024" height="329" alt="image" src="https://github.com/user-attachments/assets/ec532db7-8362-47d6-86a3-718cfe5c2833" />

