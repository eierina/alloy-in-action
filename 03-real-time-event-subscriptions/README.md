# 02 - Advanced Transaction Composition and Gas Management

## Overview

This second project delves deeper into the intricacies of Ethereum development using Rust and the Alloy framework. Building upon the foundational concepts from the first project, this example focuses on manually composing transactions, ABI encoding, and advanced gas management under EIP-1559. By leveraging `TransactionRequest`, handling nonces, calculating gas fees, and crafting function call transactions, this project provides granular control over blockchain interactions, enabling optimization and customization beyond high-level abstractions.

## Features

- **Manual Transaction Composition**: Build and configure transactions using `TransactionRequest` without relying on high-level abstractions.
- **ABI Encoding**: Manually encode contract constructor and function calls using Alloy's ABI utilities.
- **Gas Management**: Calculate and set gas parameters based on EIP-1559 specifications, including base fee and priority fee (tip).
- **Nonce Management**: Handle transaction nonces effectively, accounting for pending transactions to ensure transaction uniqueness.
- **Advanced Deployment**: Deploy smart contracts with constructor parameters by appending ABI-encoded data to deployment bytecode.
- **Transaction Confirmation Strategy**: Implement confirmation strategies to wait for a specified number of block confirmations, enhancing transaction reliability.

## Prerequisites

Ensure the following are installed and configured:

- [Rust](https://www.rust-lang.org/tools/install) (version 1.82 or later)
- [Alloy Framework](https://github.com/alloy-rs/) dependencies (included in `Cargo.toml`)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) toolchain installed
- [Anvil](https://book.getfoundry.sh/anvil) local Ethereum node running with a block time of 3 seconds:
- [Solidity 0.8.24 compiler](https://github.com/crytic/solc-select) installed
- [.env Configuration](../README.md#environment-configuration)

## Setup

1. **Clone the Repository**

   If you haven't already, clone the main repository:

   ```bash
   git clone https://github.com/eierina/alloy-in-action.git
   cd alloy-in-action
   ```

2. **Navigate to the Sub-project**

   ```bash
   cd 02-advanced-transaction-composition
   ```

3. **Configure Environment Variables**

   Ensure the root `.env` file is set up with the necessary variables:

   ```env
   # Private key for the first default Anvil account
   ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

   # RPC URL for the Anvil local Ethereum node
   ANVIL_RPC_URL=http://127.0.0.1:8545

   # WebSocket URL for the Anvil local Ethereum node
   ANVIL_WS_URL=ws://127.0.0.1:8545

   # Default Chain ID for the Anvil network
   ANVIL_CHAIN_ID=31337
   ```

4. **Start Anvil**

   Launch Anvil to provide a local Ethereum testing environment with a block time of 3 seconds:

  ```bash
  anvil --block-time 3
  ```

## Running the Project

Execute the Rust application to deploy and interact with the smart contract:

```bash
cargo run
```

### Expected Output

Upon running, you should see output similar to:

```
🔄 Transaction sent (0xc610b765f0632d08269330ce0e0fd1585a0697eb706450aa065cdac2e4730a86).
✅ Transaction confirmed (0xc610b765f0632d08269330ce0e0fd1585a0697eb706450aa065cdac2e4730a86).
🧾 Deploy transaction receipt obtained (0xc610b765f0632d08269330ce0e0fd1585a0697eb706450aa065cdac2e4730a86).
📍 Contract deployed at address (0x5fbdb2315678afecb367f032d93f642f64180aa3).
🔄 setValue transaction sent (0xbf9c3cd42e3c1b2c5313dc728e4fe401c74a743ca3535dbf9dd4c1ad5873bd49).
✅ setValue transaction confirmed (0xbf9c3cd42e3c1b2c5313dc728e4fe401c74a743ca3535dbf9dd4c1ad5873bd49).
🧾 setValue transaction receipt obtained (0xbf9c3cd42e3c1b2c5313dc728e4fe401c74a743ca3535dbf9dd4c1ad5873bd49).
🔍 Current value from contract: 2
```

## Environment Variables

The project relies on the following environment variables defined in the root `.env` file:

- `ANVIL_PRIVATE_KEY`: Private key for the Anvil account used for deploying and interacting with the contract.
- `ANVIL_RPC_URL`: RPC endpoint for the local Anvil Ethereum node.
- `ANVIL_WS_URL`: WebSocket endpoint for the local Anvil Ethereum node.
- `ANVIL_CHAIN_ID`: Chain ID for the Anvil network.

Ensure these variables are correctly set before running the project.

## License

This project is licensed under the [MIT License](../LICENSE).
