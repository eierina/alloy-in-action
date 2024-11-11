use alloy_chains::NamedChain;
use alloy_network::EthereumWallet;
use alloy_network::primitives::{BlockTransactions, BlockTransactionsKind};
use alloy_primitives::{utils, Address, TxKind, U256};
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_macro::sol;
use alloy_sol_types::{SolCall, SolConstructor};
use eyre::Result;
use url::Url;
use advanced_transaction_composition::utils::{load_environment, setup_logging};
use alloy_network::TransactionBuilder;
use alloy_rpc_types::{Block, BlockId, BlockNumberOrTag, Transaction, TransactionRequest, TransactionTrait};
use crate::SampleContract::{getValueCall};
use alloy_sol_types::private::Bytes;
use utils::{format_ether, format_units, parse_units};

// ASSUMPTIONS: Ethereum, EIP-1559 only

sol! {
    // source/reference contract in solidity-smart-contracts/src/SampleContract.sol
    // bytecode via `solc SampleContract.sol --bin --via-ir --optimize --optimize-runs 1`
    #[sol(rpc, bytecode = "608034604d57601f61024238819003918201601f19168301916001600160401b03831184841017605157808492602094604052833981010312604d57515f556040516101dc90816100668239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c90816312065fe01461018e5750806320965255146101375780633ccfd60b146101535780633fa4f2451461013757806355241077146100f457806357eca1a5146100a95763d0e30db014610069575f80fd5b5f3660031901126100a5577f1e57e3bb474320be3d2c77138f75b7c3941292d647f5f9634e33a8e94e0e069b60408051338152346020820152a1005b5f80fd5b346100a5575f3660031901126100a5576040516335fdd7ab60e21b815260206004820152601260248201527168656c6c6f2066726f6d207265766572742160701b6044820152606490fd5b346100a55760203660031901126100a5577f93fe6d397c74fdf1402a8b72e47b68512f0510d7b98a4bc4cbdf6ac7108b3c596020600435805f55604051908152a1005b346100a5575f3660031901126100a55760205f54604051908152f35b346100a5575f3660031901126100a5575f80808047818115610185575b3390f11561017a57005b6040513d5f823e3d90fd5b506108fc610170565b346100a5575f3660031901126100a557602090478152f3fea2646970667358221220b4a67dc718859dcd100786802486745715317198aba986b0fa547130f8a19cd164736f6c634300081b0033")]
    contract SampleContract {
        uint256 public value;

        event ValueChanged(uint256 newValue);
        event EtherReceived(address sender, uint256 amount);

        error SampleError(string message);

        constructor(uint256 _initialValue);

        function setValue(uint256 _value) external;
        function getValue() external view returns (uint256 currentValue);
        function deposit() external payable;
        function withdraw() external;
        function getBalance() external view returns (uint256 balance);
        function revertWithError() external;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    load_environment()?;
    setup_logging();

    // Create signer and wallet
    let private_key = std::env::var("ANVIL_PRIVATE_KEY")?;
    let signer: PrivateKeySigner = private_key.parse()?;
    let signer_address = signer.address();
    let wallet: EthereumWallet = EthereumWallet::from(signer);

    // Configure provider URL
    let ws_url = Url::parse(&std::env::var("ANVIL_WS_URL")?)?;
    let rpc_url = Url::parse(&std::env::var("ANVIL_RPC_URL")?)?;
    // let ws_url = Url::parse("ws://192.168.0.188:8546")?;
    // let rpc_url = Url::parse("http://192.168.0.188:8545")?;

    // Set up provider with chain ID, wallet, and network details
    let provider = ProviderBuilder::new()
        .with_chain_id(31337) // anvil-hardhat chain ID - set here for all transaction with this provider, or set on transactionrequest for
        .with_chain(NamedChain::AnvilHardhat)
        .wallet(wallet)
        .on_ws(WsConnect::new(ws_url)).await?;

    // Set the number of confirmations to wait for a transaction to be "confirmed"
    // (6-12) for high value transactions, (1-3) for low value transactions
    let confirmations = 3u64;

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

    // Fixed tip of 2.5 Gwei for all transactions
    let tip = parse_units("2.5", "gwei")?.try_into()?;

    // Prepare contract deployment bytecode with initialization of value to 1
    let initial_value = U256::from(1);
    let deploy_bytecode: Bytes = [
        &SampleContract::BYTECODE[..],
        &SampleContract::constructorCall { _initialValue: initial_value }.abi_encode()[..],
    ]
        .concat()
        .into();

    let nonce = provider.get_transaction_count(signer_address).pending().await?;

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
    let gas_delta_abs = gas_delta.abs() as u64;

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

