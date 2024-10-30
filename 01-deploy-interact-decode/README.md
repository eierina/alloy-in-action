# 01 - Deploy, Interact, Decode.

## Overview

This first project demonstrates the fundamental operations of connecting to a node, deploying a Solidity smart contract, interacting with its functions, handling events, and managing custom errors on the Ethereum blockchain. Leveraging the Alloy framework, this example showcases seamless integration between Rust and Solidity, enabling efficient blockchain development.

## Features

- **Contract Deployment**: Deploy the [`SampleContract`](../solidity-smart-contracts/src/SampleContract.sol) with an initial state.
- **State Interaction**: Read and update contract state variables.
- **Event Handling**: Handle and decode emitted events from transactions.
- **Error Management**: Handle and decode custom contract errors.
- **Automated Type Generation**: Utilize the `sol!` macro to generate Rust types from Solidity contracts.

## Prerequisites

Ensure the following are installed and configured:

- [Rust](https://www.rust-lang.org/tools/install) (version 1.82 or later)
- [Alloy Framework](https://github.com/alloy-rs/) dependencies (included in `Cargo.toml`)
- [Anvil](https://book.getfoundry.sh/anvil) local Ethereum node
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
   cd 01-deploy-interact-decode
   ```

3. **Configure Environment Variables**

   Ensure the root `.env` file is set up with the necessary variables:

   ```env
   # Private key for the first default Anvil account
   ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

   # RPC URL for the Anvil local Ethereum node
   ANVIL_RPC_URL=http://127.0.0.1:8545

   # Optional: Chain ID for the Anvil network
   ANVIL_CHAIN_ID=31337
   ```

4. **Start Anvil**

   Launch Anvil to provide a local Ethereum testing environment:

   ```bash
   anvil
   ```

## Running the Project

Execute the Rust application to deploy and interact with the smart contract:

```bash
cargo run
```

### Expected Output

Upon running, you should see output similar to:

```
📦 Contract deployed with initial value: 1
🔍 Initial value retrieved from contract: 1
🔄 Transaction sent to set new value. Transaction hash: 0x523975fa69b24c2e273f70e44fe81dc601f0b0713dc3ff132cf654958f3a8b82
🧾 Transaction receipt obtained. Receipt hash: 0x523975fa69b24c2e273f70e44fe81dc601f0b0713dc3ff132cf654958f3a8b82
⚡️ Event: ValueChanged - newValue: 2
🔍 Updated value retrieved from contract: 2
🔍 Initial contract balance: 0.000000000000000000 Ξ
🔍 Initial signer balance: 9999.990342325640506152 Ξ
🔄 Transaction sent to deposit Ether. Transaction hash: 0xdc5e49931f9ecabe1baa25eab91e83ce7113ab9606eddf1de7264a563b432f7c
🧾 Transaction receipt obtained. Receipt hash: 0xdc5e49931f9ecabe1baa25eab91e83ce7113ab9606eddf1de7264a563b432f7c
⚡️ Event: EtherReceived - sender: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266; amount: 1000000000000000
🔍 Contract balance after deposit: 0.001000000000000000 Ξ
🔍 Signer balance after deposit: 9999.989342002897191564 Ξ
⚠️ Call reverted with SampleError: "hello from revert!"
```

## Environment Variables

The project relies on the following environment variables defined in the root `.env` file:

- `ANVIL_PRIVATE_KEY`: Private key for the Anvil account used for deploying and interacting with the contract.
- `ANVIL_RPC_URL`: RPC endpoint for the local Anvil Ethereum node.
- `ANVIL_CHAIN_ID`: Chain ID for the Anvil network.

Ensure these variables are correctly set before running the project.

## License

This project is licensed under the [MIT License](../LICENSE).
