# Solidity Smart Contracts

## Overview

This directory houses the Solidity smart contracts utilized in the Alloy in Action Rust examples. It includes the `SampleContract` along with configurations for compilation and testing using Foundry. As the series progresses, additional contracts and corresponding tests will be added to demonstrate various Solidity features and best practices.

## SampleContract.sol

The `SampleContract` is a simple Solidity contract that demonstrates basic features such as state variables, events, and error handling. It serves as the primary contract interacted with in the Rust examples.

### Source Code

Located at `src/SampleContract.sol`.

## Compilation

The contracts are compiled using [Foundry](https://getfoundry.sh/). To compile the contracts:

1. **Install Foundry**

   If you haven't installed Foundry, follow the [installation guide](https://book.getfoundry.sh/getting-started/installation).

2. **Navigate to the Contracts Directory**

   ```bash
   cd solidity-smart-contracts
   ```

3. **Compile the Contracts**

   ```bash
   forge build
   ```

   This command compiles the Solidity contracts and generates the necessary artifacts in the `out/` directory.

## Testing

Solidity tests are written to ensure the correctness of the contracts.

1. **Run Tests**

   ```bash
   forge test
   ```

   This command executes all tests located in the `test/` directory.

## License

This project is licensed under the [MIT License](../LICENSE).
