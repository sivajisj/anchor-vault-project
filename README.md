# Blueshift Anchor Vault

A Solana program built with Anchor framework that implements a simple vault system where users can deposit and withdraw SOL securely using Program Derived Addresses (PDAs).

## 📋 Overview

The Blueshift Anchor Vault allows users to:
- **Deposit SOL** into a personalized vault account
- **Withdraw SOL** from their vault back to their wallet
- Each user gets their own vault PDA for secure fund management

## 🏗️ Program Structure

### Instructions

1. **`deposit(amount: u64)`**
   - Transfers SOL from user's wallet to their vault PDA
   - Ensures deposit amount covers rent exemption
   - Creates vault PDA if it doesn't exist

2. **`withdraw(amount: u64)`**
   - Transfers SOL from vault PDA back to user's wallet
   - Requires PDA signature for authorization
   - Ensures sufficient balance in vault

### Accounts

- **`VaultAction`** context used for both deposit and withdraw:
  - `signer` - The user performing the operation
  - `vault` - PDA vault account (seeds: `["vault", signer.key]`)
  - `system_program` - System program for SOL transfers

## 🚀 Getting Started

### Prerequisites

```bash
# Required tools
rustup install stable
solana-install init 1.17.0
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.29.0
avm use 0.29.0
