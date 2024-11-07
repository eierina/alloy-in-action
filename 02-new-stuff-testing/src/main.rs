use alloy_chains::NamedChain;
use alloy_network::EthereumWallet;
use alloy_primitives::{TxKind, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_macro::sol;
use alloy_sol_types::{SolCall, SolConstructor};
use eyre::Result;
use url::Url;
use new_stuff_testing::utils::{load_environment, setup_logging};
use alloy_network::TransactionBuilder;
use alloy_rpc_types::TransactionRequest;
use crate::SampleContract::{getValueCall};
use alloy_sol_types::private::Bytes;

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

    // Configure provider
    let ws_url = Url::parse(&std::env::var("ANVIL_WS_URL")?)?;
    let rpc_url = Url::parse(&std::env::var("ANVIL_RPC_URL")?)?;

    // Set up provider
    let provider = ProviderBuilder::new()
        .with_chain_id(31337) // set here for all transaction with this provider, or set on transactionrequest for
        .wallet(wallet)
        .with_chain(NamedChain::AnvilHardhat)
        .on_http(rpc_url);
        //.on_ws(WsConnect::new(ws_url)).await?;

    let confirmations = 1u64;

    // Fetch the latest block to obtain base fee
    // let latest_block = provider
    //     .get_block(BlockId::latest(), BlockTransactionsKind::Hashes)
    //     .await?
    //     .unwrap();
    //
    // let base_fee_per_gas = latest_block.header.base_fee_per_gas.unwrap();

    // Prepare contract deployment with initial value of 1
    let initial_value = U256::from(1);
    let deploy_bytecode: Bytes = [
        &SampleContract::BYTECODE[..],
        &SampleContract::constructorCall { _initialValue: initial_value }.abi_encode()[..],
    ]
        .concat()
        .into();

    let nonce = provider.get_transaction_count(signer_address).pending().await?;

    let tx = TransactionRequest::default()
        .with_deploy_code(deploy_bytecode)
        // .with_input(deploy_bytecode)
        // .with_to(Address::ZERO)
        // .with_kind(TxKind::Create)
        .with_nonce(nonce)
        .with_gas_limit(21_000_000)
        .with_max_priority_fee_per_gas(1_000_000_000_000)
        .with_max_fee_per_gas(20_000_000_000_000)
        ;

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
    println!("🏷️ Contract deployed at address ({:#x}).", deploy_address);

    // Prepare setValue transaction to update the value to 2
    let tx_data = SampleContract::setValueCall { _value: U256::from(2u64) }.abi_encode();
    let nonce = provider.get_transaction_count(signer_address).pending().await?;

    let tx = TransactionRequest::default()
        .with_input(tx_data)
        .with_to(deploy_address)
        .with_from(signer_address)
        .with_nonce(nonce)
        .with_kind(TxKind::Call(deploy_address)) // ?
        .with_gas_limit(21_000_000)
        .with_max_priority_fee_per_gas(1_000_000_000_000)
        .with_max_fee_per_gas(20_000_000_000_000)
        ;

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

    //let contract = SampleContract::new(deploy_address, provider.clone());

/*
Breakdown by Confirmation Count
1 Confirmation: The transaction is in the blockchain but may be at risk from a reorganization (reorg). Suitable only for low-value or low-risk transactions.

6 Confirmations: Often considered reasonably secure for lower-value transactions. This number is common for exchanges and applications with moderate security needs, providing a balance between speed and security.

12 Confirmations: This is widely considered secure, as the likelihood of reorgs impacting the transaction drops significantly. For most use cases, this is the standard for high-value transactions.

24+ Confirmations: For very high-value or critical transactions, some parties may wait for 24 confirmations or more. This adds extra safety, though it’s rare to require this many confirmations.

Practical Recommendation
In summary:

12 confirmations is usually safe and is a common standard for Ethereum mainnet.
Adjust up or down based on the transaction’s value and risk sensitivity.
*/
    // println!("Sending tx: {:?}", tx);
    // let tx_builder = provider.send_transaction(tx).await?; // eth_sendRawTransaction
    // let sent_hash = *tx_builder.tx_hash();
    // println!("Sent tx: {:?}", sent_hash);
    // let pending_tx = tx_builder.with_required_confirmations(12).register().await?; // eth_getTransactionReceipt
    // println!("Got pending tx");
/* WS:
eth_chainId
eth_getTransactionCount
eth_getBlockByNumber
eth_sendRawTransaction
eth_getTransactionReceipt
*/

/* HTTP
eth_chainId
eth_getTransactionCount
eth_getBlockByNumber
eth_sendRawTransaction
eth_blockNumber
eth_getTransactionReceipt
eth_getBlockByNumber
*/


    Ok(())
}

