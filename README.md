# Alloy in Action

Alloy in Action is a comprehensive repository designed to demonstrate practical methods for integrating Rust applications with blockchain technologies. Serving as the foundation for the "Alloy in Action" [blog series](https://block-zero.io/blog/alloy-in-action/introduction), this project guides you through various facets of Rust-based blockchain client development, offering hands-on examples and in-depth explanations to enhance your understanding and proficiency.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.82+-lightgray.svg)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Environment Configuration](#environment-configuration)
- [Rust Projects](#rust-projects)
- [Solidity Smart Contracts](#solidity-smart-contracts)
- [Running the Examples](#running-the-examples)
- [Contributing](#contributing)
- [License](#license)

## Overview

This repository is designed to showcase how Rust can be effectively used to interact with Ethereum smart contracts. Through a series of Rust sub-projects, you will learn how to:

- Deploy Solidity smart contracts
- Interact with contract functions and state
- Handle events and custom errors
- Automate contract interactions using Rust macros
- Extend functionalities with advanced Rust features

## Getting Started

### Prerequisites

Ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (version 1.82 or later)
- [Foundry](https://getfoundry.sh/) (for Solidity development and testing)
- [Anvil](https://book.getfoundry.sh/anvil) (local Ethereum node for testing)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
- [Git](https://git-scm.com/)

### Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/eierina/alloy-in-action.git
   cd alloy-in-action
   ```

2. **Install Dependencies**

   Ensure you have Rust and Foundry installed. You can install Foundry by following the [official guide](https://book.getfoundry.sh/getting-started/installation).

3. **Set Up Environment Variables**

   Check the `.env` file is in the root directory and correctly configured. See [environment configuration](#environment-configuration) section.

4. **Start Anvil**

   Launch a local Ethereum node using Anvil:

   ```bash
   anvil
   ```
   Anvil will start on `http://127.0.0.1:8545` by default.

### Environment Configuration

The `.env` file is located in the root directory with the following non-sensitive content:

   ```env
   # Private key for the first default Anvil account
   ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

   # RPC URL for the Anvil local Ethereum node
   ANVIL_RPC_URL=http://127.0.0.1:8545

   # Optional: Chain ID for the Anvil network
   ANVIL_CHAIN_ID=31337
   ```

## Rust Projects

### 01-deploy-interact-decode

The first Rust sub-project demonstrates how to:

- Deploy a Solidity smart contract
- Interact with contract functions (reading and writing state)
- Handle events and decode logs
- Manage custom contract errors
- Utilize the `sol!` macro to generate Rust types from Solidity contracts

The related blog post can be found [here](https://block-zero.io/blog/alloy-in-action/introduction).

### 02-coming-soon

Additional examples and be added as the series progresses.

## Solidity Smart Contracts

The `solidity-smart-contracts` directory contains:

- **SampleContract.sol**: A sample Solidity contract used in the Rust examples.
- **Tests**: Solidity tests to ensure contract correctness.

Additional contracts and tests will be added as the series progresses.

## Running the Examples

Navigate to the `01-deploy-interact-decode` directory and run the Rust project:

```bash
cd 01-deploy-interact-decode
cargo run
```

Ensure that Anvil is running and the `.env` file is properly configured.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to enhance the project.

## License

This project is licensed under the [MIT License](LICENSE).
