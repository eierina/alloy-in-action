# In-ReVM-We-Rust: Alloy-rs Interaction Example

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-lightgray.svg)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)

This repository serves as the foundational sub-project for our series on interacting with Ethereum smart contracts using Rust. In this first example, we'll demonstrate how to use **alloy-rs** to connect to an **Anvil** instance, deploy a sample smart contract, and perform various interactions such as reading and updating contract state, handling Ether deposits, decoding events, and managing custom revert errors.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Sample Output](#sample-output)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Overview

This sub-project provides a comprehensive example of how to leverage **alloy-rs**, a Rust library for Ethereum smart contract interactions, to perform the following tasks:

1. **Connect to an Anvil Instance:** Establish a connection to a local Ethereum development node using Anvil.
2. **Deploy a Smart Contract:** Deploy a sample Solidity contract (`SampleContract`) with an initial state.
3. **Interact with the Contract:**
    - **Read State:** Retrieve the current value stored in the contract.
    - **Update State:** Change the contract's state via a transaction.
    - **Handle Ether Deposits:** Deposit Ether into the contract and verify balance changes.
    - **Decode Events:** Listen for and decode emitted events from contract interactions.
    - **Manage Custom Revert Errors:** Invoke a contract function that triggers a custom revert error and decode the error message.

The accompanying blog post delves into the details of each step, providing insights and explanations to enhance your understanding.

## Features

- **Smart Contract Deployment:** Automate the deployment process of a Solidity smart contract.
- **State Management:** Easily read and modify the contract's state variables.
- **Ether Transactions:** Handle Ether transfers to and from the contract.
- **Event Handling:** Decode and process events emitted by the contract.
- **Error Decoding:** Capture and interpret custom revert errors from contract calls.

## Prerequisites

Before getting started, ensure you have the following installed on your system:

- **Rust:** [Install Rust](https://www.rust-lang.org/tools/install) (version 1.70 or later recommended)
- **Anvil:** [Anvil Installation Guide](https://book.getfoundry.sh/getting-started/installation.html#anvil)
- **Solidity Compiler (`solc`):** [Installation Instructions](https://docs.soliditylang.org/en/v0.8.19/installing-solidity.html)

## Installation

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/in-revm-we-rust.git
   cd in-revm-we-rust/first-subproject
   ```

2. **Install Dependencies:**

   Ensure you have the required Rust crates by running:

   ```bash
   cargo build
   ```

   This command will fetch and compile all necessary dependencies specified in the `Cargo.toml` file.

## Configuration

1. **Set Up Environment Variables:**

   Create a `.env` file in the root directory of the sub-project with the following content:

   ```env
   ANVIL_PRIVATE_KEY=your_private_key_here
   ANVIL_RPC_URL=http://127.0.0.1:8545
   ```

    - **`ANVIL_PRIVATE_KEY`:** The private key of the account you wish to use for deploying and interacting with the contract. **Ensure this key is kept secure and never shared.**
    - **`ANVIL_RPC_URL`:** The RPC endpoint URL for your Anvil instance.

2. **Start Anvil:**

   Launch Anvil in a separate terminal window:

   ```bash
   anvil
   ```

   Anvil will start a local Ethereum node listening on `http://127.0.0.1:8545` by default.

## Usage

Run the Rust application using Cargo:

```bash
cargo run
```

This command will execute the `main` function, performing the following actions sequentially:

1. **Deploy the `SampleContract`** with an initial value of `1`.
2. **Retrieve and display** the initial value from the contract.
3. **Set the contract's value** to `2` and display the transaction hash.
4. **Fetch the transaction receipt**, decode, and handle the `ValueChanged` event.
5. **Verify and display** the updated contract value.
6. **Deposit `1 Milli-Ether`** into the contract and display relevant transaction details.
7. **Fetch and decode** the `EtherReceived` event from the deposit transaction.
8. **Retrieve and display** the updated contract and signer balances.
9. **Invoke `revertWithError`** to trigger and handle a custom revert error (`SampleError`).

## Sample Output

Upon successful execution, you should observe output similar to the following:

```plaintext
📦 Contract deployed with initial value: 1
🔍 Initial value retrieved from contract: 1
🔄 Transaction sent to set new value. Transaction hash: 0xc79c2fc12f0f082ea64329538a848493658e8288806a494d4b9b94fae9ac3fc5
🧾 Transaction receipt obtained. Receipt hash: 0xc79c2fc12f0f082ea64329538a848493658e8288806a494d4b9b94fae9ac3fc5
⚡️ Event: ValueChanged - newValue: 2
🔍 Updated value retrieved from contract: 2
🔍 Initial contract balance: 0.000000000000000000 Ξ
🔍 Initial signer balance: 9999.995397095296036362 Ξ
🔄 Transaction sent to deposit Ether. Transaction hash: 0xc698c428778e4536ca7b68cd4140d50f33724a0b1acfe50c007dcf759df0d4c3
🧾 Transaction receipt obtained. Receipt hash: 0xc698c428778e4536ca7b68cd4140d50f33724a0b1acfe50c007dcf759df0d4c3
⚡️ Event: EtherReceived - sender: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266; amount: 1000000000000000
🔍 Contract balance after deposit: 0.001000000000000000 Ξ
🔍 Signer balance after deposit: 9999.994394729276504910 Ξ
⚠️ Call reverted with SampleError: "failed"

Ok, my first in-revm-we-rust blog post source code is ready.
```

**Note:** The exact values, especially transaction hashes and addresses, will vary based on your local Anvil setup and the specific accounts used.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

- [alloy-rs](https://github.com/Project-OSRM/alloy-rs) - A Rust library for Ethereum smart contract interactions.
- [Anvil](https://github.com/foundry-rs/foundry) - A blazing fast Ethereum development node.
- [Solidity](https://soliditylang.org/) - The contract-oriented programming language used for writing smart contracts.
- [Rust Programming Language](https://www.rust-lang.org/) - Empowering everyone to build reliable and efficient software.

---

Stay tuned for our upcoming blog posts, where we'll explore each of these interactions in depth, providing detailed explanations and best practices for Rust developers venturing into Ethereum smart contract development!