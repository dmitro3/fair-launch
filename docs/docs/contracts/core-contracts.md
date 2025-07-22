---
sidebar_position: 1
---

# 🔧 Core Contracts

## Program IDs

| Environment   | Program ID                                   | Solscan Link                                                                                                   | Source Code                                                                                                   |
|--------------|-----------------------------------------------|----------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|
| Testnet      | `CB18NKSvKunD2xeuvEkKfBxuz4fJFJJ8GPy5w1dMzN1` | <a href="https://solscan.io/account/CB18NKSvKunD2xeuvEkKfBxuz4fJFJJ8GPy5w1dMzN1?cluster=devnet" target="_blank" rel="noopener noreferrer">View on Solscan</a>     | <a href="https://github.com/PotLock/fair-launch/tree/feat/add-migrate/solana-contract" target="_blank" rel="noopener noreferrer">View on GitHub</a> |
| Staging      | `CB18NKSvKunD2xeuvEkKfBxuz4fJFJJ8GPy5w1dMzN1` | <a href="https://solscan.io/account/CB18NKSvKunD2xeuvEkKfBxuz4fJFJJ8GPy5w1dMzN1?cluster=devnet" target="_blank" rel="noopener noreferrer">View on Solscan</a>     | <a href="https://github.com/PotLock/fair-launch/tree/feat/add-migrate/solana-contract" target="_blank" rel="noopener noreferrer">View on GitHub</a> |
| Production   | `CB18NKSvKunD2xeuvEkKfBxuz4fJFJJ8GPy5w1dMzN1` | <a href="https://solscan.io/account/CB18NKSvKunD2xeuvEkKfBxuz4fJFJJ8GPy5w1dMzN1" target="_blank" rel="noopener noreferrer">View on Solscan</a>                    | <a href="https://github.com/PotLock/fair-launch/tree/feat/add-migrate/solana-contract" target="_blank" rel="noopener noreferrer">View on GitHub</a> |

The Program ID is the primary Solana program currently deployed in the POTLAUNCH protocol. It serves as the main entry point for token creation and management.

### Key Features

- **Token Deployment:** Handles the creation and deployment of new tokens
- **Protocol Fee Management:** Manages and collects protocol fees from token launches

### Deployment Processes

The program streamlines the token deployment process by:
- Validating token parameters
- Managing deployment permissions
- Ensuring proper initialization of new tokens

### Protocol Fee Management

- Collects fees from successful token launches
- Distributes fees according to protocol governance
- Maintains transparent fee structure

## Future Contracts

The following contracts are planned for future deployment:

### Governance Contract

- Proposal creation and voting mechanisms
- Execution mechanisms for governance decisions
- Security features for decentralized governance

### Bonding Curve Contract

- Dynamic token pricing algorithms
- Liquidity addition and removal
- Advanced fee management systems 