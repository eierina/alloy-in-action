use std::path::Path;
use alloy_chains::NamedChain;
use alloy_network::EthereumWallet;
use alloy_network::primitives::BlockTransactionsKind;
use alloy_primitives::{utils, TxKind, U256};
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_macro::sol;
use alloy_sol_types::{SolCall, SolConstructor};
use eyre::Result;
use url::Url;
use alloy_network::TransactionBuilder;
use alloy_rpc_types::{BlockId, TransactionRequest};
use crate::SampleContract::{getValueCall};
use alloy_sol_types::private::Bytes;
use utils::parse_units;

sol! {
    // source/reference contract in solidity-smart-contracts/src/SampleContract.sol
    // bytecode via `solc SampleContract.sol --bin --via-ir --optimize --optimize-runs 1`
    #[sol(rpc, bytecode = "608034604d57601f61028038819003918201601f19168301916001600160401b03831184841017605157808492602094604052833981010312604d57515f5560405161021a90816100668239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c90816312065fe0146101cc5750806320965255146101405780633ccfd60b1461015c5780633fa4f2451461014057806355241077146100f857806357eca1a5146100ad5763d0e30db014610069575f80fd5b5f3660031901126100a957476040519034825260208201527f1d57945c1033a96907a78f6e0ebf6a03815725dac25f33cc806558670344ac8860403392a2005b5f80fd5b346100a9575f3660031901126100a9576040516335fdd7ab60e21b815260206004820152601260248201527168656c6c6f2066726f6d207265766572742160701b6044820152606490fd5b346100a95760203660031901126100a9576004355f5490805f556040519081527fe435f0fbe584e62b62f48f4016a57ef6c95e4c79f5babbe6ad3bb64f3281d26160203392a3005b346100a9575f3660031901126100a95760205f54604051908152f35b346100a9575f3660031901126100a95747805f81156101c3575b5f80809381933390f1156101b8576040519081525f60208201527fd5ca65e1ec4f4864fea7b9c5cb1ec3087a0dbf9c74641db3f6458edf445c405160403392a2005b6040513d5f823e3d90fd5b506108fc610176565b346100a9575f3660031901126100a957602090478152f3fea2646970667358221220cae439afc02e7259cc99c579d322222052f82f79b377ffd437d0523157cb795f64736f6c634300081b0033")]
    contract SampleContract {
        // Events
        event ValueChanged(address indexed updater, uint256 indexed oldValue, uint256 newValue);
        event EtherReceived(address indexed sender, uint256 amount, uint256 newBalance);
        event EtherWithdrawn(address indexed recipient, uint256 amount, uint256 remainingBalance);

        // Errors
        error SampleError(string cause);

        // Constructor
        constructor(uint256 _initialValue);

        // Functions
        /// @notice Sets a new value for the 'value' state variable
        /// @param _value The new value to be set
        function setValue(uint256 _value) external;

        /// @notice Retrieves the current value of the 'value' state variable
        /// @return currentValue The current value stored in 'value'
        function getValue() external view returns (uint256 currentValue);

        /// @notice Accepts Ether deposits and logs the sender and amount
        function deposit() external payable;

        /// @notice Withdraws the entire balance of the contract to the caller
        function withdraw() external;

        /// @notice Retrieves the contract's current Ether balance
        /// @return balance The current balance of the contract in wei
        function getBalance() external view returns (uint256 balance);

        /// @notice Reverts the transaction with a custom error message
        /// @dev Used to demonstrate custom error handling in Solidity
        function revertWithError() external pure;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load root .env and initialize environment variables
    let env_path =
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(".env");
    dotenv::from_path(env_path).ok();

    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Create signer and wallet
    let private_key = std::env::var("ANVIL_PRIVATE_KEY")?;
    let signer: PrivateKeySigner = private_key.parse()?;
    let signer_address = signer.address();
    let wallet: EthereumWallet = EthereumWallet::from(signer);

    // Set up provider with chain ID, wallet, and network details (using WebSocket)
    let ws_url = std::env::var("ANVIL_WS_URL")?;
    let ws_url = Url::parse(&ws_url)?;
    let provider = ProviderBuilder::new()
        .with_chain(NamedChain::AnvilHardhat)
        .with_chain_id(31337)
        .wallet(wallet)
        .on_ws(WsConnect::new(ws_url)).await?;

    // Set the number of confirmations to wait for a transaction to be "confirmed"
    // (6-12) for high value transactions, (1-3) for low value transactions
    let confirmations = 3u64;

    // Prepare contract deployment bytecode with initialization of value to 1
    let initial_value = U256::from(1);
    let deploy_bytecode: Bytes = [
        &SampleContract::BYTECODE[..],
        &SampleContract::constructorCall { _initialValue: initial_value }.abi_encode()[..],
    ]
        .concat()
        .into();

    let nonce = provider.get_transaction_count(signer_address).pending().await?;

    // Fetch the latest block to obtain current gas parameters
    let latest_block = provider
        .get_block(BlockId::latest(), BlockTransactionsKind::Hashes)
        .await?
        .unwrap();

    // Calculate next block's base fee based on the latest block
    let base_fee = calculate_base_fee_per_gas(
        latest_block.header.base_fee_per_gas.unwrap(),
        latest_block.header.gas_used,
        latest_block.header.gas_limit
    );

    // We set a fixed tip of 2.5 Gwei for simplicity.
    let tip = parse_units("2.5", "gwei")?.try_into()?;

    let tx_base = TransactionRequest::default()
        .with_deploy_code(deploy_bytecode)
        // .with_input(deploy_bytecode)
        // .with_kind(TxKind::Create)
        .with_nonce(nonce);

    let estimated_gas = provider.estimate_gas(&tx_base).await?;

    let tx = tx_base.with_gas_limit(estimated_gas)
        .with_max_priority_fee_per_gas(tip)
        .with_max_fee_per_gas(base_fee as u128 + tip);

    // Send deployment transaction
    let tx_builder = provider.send_transaction(tx).await?; // eth_sendRawTransaction
    println!("🔄 Transaction sent ({:#x}).", tx_builder.tx_hash());

    // Await confirmation
    let tx_hash = tx_builder.with_required_confirmations(confirmations).watch().await?;
    // NOTE - watch is equivalent and replaces the two lines below (leave this comment, but you can improve it)
    // let pending_tx = tx_builder.with_required_confirmations(confirmations).register().await?;
    // let tx_hash = pending_tx.await?;
    println!("✅ Transaction confirmed ({:#x}).", tx_hash);

    // Retrieve transaction receipt
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("Deploy transaction receipt not found");
    println!("🧾 Deploy transaction receipt obtained ({:#x}).", receipt.transaction_hash);

    let deploy_address = receipt.contract_address.unwrap();
    println!("📍 Contract deployed at address ({:#x}).", deploy_address);

    // Fetch the latest block to obtain current block gas parameters
    let latest_block = provider
        .get_block(BlockId::latest(), BlockTransactionsKind::Hashes)
        .await?
        .unwrap();

    // calculate next block's base fee
    let base_fee = calculate_base_fee_per_gas(
        latest_block.header.base_fee_per_gas.unwrap(),
        latest_block.header.gas_used,
        latest_block.header.gas_limit
    );

    // Prepare setValue transaction to update the value to 2
    let tx_data = SampleContract::setValueCall { _value: U256::from(2u64) }.abi_encode();
    let nonce = provider.get_transaction_count(signer_address).pending().await?;

    let tx_base = TransactionRequest::default()
        .with_input(tx_data)
        .with_to(deploy_address)
        .with_from(signer_address)
        .with_nonce(nonce)
        .with_kind(TxKind::Call(deploy_address));

    let estimated_gas = provider.estimate_gas(&tx_base).await?;

    let tx = tx_base.with_gas_limit(estimated_gas)
        .with_max_priority_fee_per_gas(tip)
        .with_max_fee_per_gas(base_fee as u128 + tip);

    // Send setValue transaction
    let tx_builder = provider.send_transaction(tx).await?;
    println!("🔄 setValue transaction sent ({:#x}).", tx_builder.tx_hash());

    // Await confirmation
    let tx_hash = tx_builder.with_required_confirmations(confirmations).watch().await?;
    println!("✅ setValue transaction confirmed ({:#x}).", tx_hash);

    // Retrieve transaction receipt for setValue
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("setValue transaction receipt not found");
    println!("🧾 setValue transaction receipt obtained ({:#x}).", receipt.transaction_hash);

    // Prepare getValue call to fetch the current value
    let tx_data = SampleContract::getValueCall { }.abi_encode();
    let tx = TransactionRequest::default()
        .with_input(tx_data)
        .with_to(deploy_address)
        .with_from(signer_address)
        .with_kind(TxKind::Call(deploy_address))
        ;

    // Execute getValue call
    let result = provider.call(&tx).await?;
    let decoded_value = getValueCall::abi_decode_returns(&result, true)?;
    let current_value = decoded_value.currentValue;

    println!("🔍 Current value from contract: {}", current_value);

    Ok(())
}

/// Calculates the base fee per gas for the next block based on EIP-1559 specifications.
///
/// This function adjusts the base fee according to the gas usage of the current block.
/// If the gas used is higher than the target (50% of the gas limit), the base fee increases.
/// If it's lower, the base fee decreases. The change is capped at a maximum of ±12.5% per block.
///
/// # Arguments
///
/// * `current_base_fee` - The base fee per gas of the current block (in wei).
/// * `current_gas_used` - The total gas used in the current block.
/// * `current_gas_limit` - The gas limit of the current block.
///
/// # Returns
///
/// * `u64` - The calculated base fee per gas for the next block.
pub fn calculate_base_fee_per_gas(
    current_base_fee: u64,
    current_gas_used: u64,
    current_gas_limit: u64,
) -> u64 {
    // Calculate the target gas usage (50% of the gas limit)
    let gas_target = current_gas_limit / 2;

    // Calculate the difference between gas used and gas target
    let gas_delta = current_gas_used as i64 - gas_target as i64;

    // Maximum base fee change is 12.5% of the current base fee
    let max_base_fee_change = current_base_fee / 8;

    // If gas usage is exactly at the target, base fee remains the same
    if gas_delta == 0 {
        return current_base_fee;
    }

    // Calculate the absolute value of gas delta for adjustment calculation
    let gas_delta_abs = gas_delta.unsigned_abs();

    // Compute the base fee change
    // Using u128 to prevent potential overflow in intermediate calculations
    let base_fee_change = ((max_base_fee_change as u128 * gas_delta_abs as u128)
        / gas_target as u128) as u64;

    if gas_delta > 0 {
        // Increase base fee by the calculated change
        current_base_fee + base_fee_change
    } else {
        // Decrease base fee by the calculated change, ensuring it doesn't go below zero
        if current_base_fee > base_fee_change {
            current_base_fee - base_fee_change
        } else {
            0
        }
    }
}

