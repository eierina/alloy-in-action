use std::ops::Div;
use alloy_contract::Error;
use alloy_network::EthereumWallet;
use alloy_primitives::{utils, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_macro::sol;
use alloy_sol_types::SolEventInterface;
use utils::format_ether;
use eyre::Result;
use tokio::main;
use tracing_subscriber;
use url::Url;
use crate::SampleContract::{SampleContractErrors};

sol! {
    // bytecode via `solc SampleContract.sol --bin --via-ir --optimize --optimize-runs 1`
    #[sol(rpc, bytecode = "608034604d57601f61023638819003918201601f19168301916001600160401b03831184841017605157808492602094604052833981010312604d57515f556040516101d090816100668239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c90816312065fe01461018257508063209652551461012b5780633ccfd60b146101475780633fa4f2451461012b57806355241077146100e857806357eca1a5146100a95763d0e30db014610069575f80fd5b5f3660031901126100a5577f1e57e3bb474320be3d2c77138f75b7c3941292d647f5f9634e33a8e94e0e069b60408051338152346020820152a1005b5f80fd5b346100a5575f3660031901126100a5576040516335fdd7ab60e21b815260206004820152600660248201526519985a5b195960d21b6044820152606490fd5b346100a55760203660031901126100a5577f93fe6d397c74fdf1402a8b72e47b68512f0510d7b98a4bc4cbdf6ac7108b3c596020600435805f55604051908152a1005b346100a5575f3660031901126100a55760205f54604051908152f35b346100a5575f3660031901126100a5575f80808047818115610179575b3390f11561016e57005b6040513d5f823e3d90fd5b506108fc610164565b346100a5575f3660031901126100a557602090478152f3fea26469706673582212202d25e897557c571fb49c1a6e70b25c3e72a319b755fcc749669bf122a33e3d5a64736f6c634300081b0033")]
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

#[main]
async fn main() -> Result<()> {
    // Initialize environment and dependencies
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Create signer and wallet
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer: PrivateKeySigner = private_key.parse()?;
    let signer_address = signer.address();
    let wallet = EthereumWallet::from(signer);

    // Set up provider
    let rpc_url = Url::parse("http://127.0.0.1:8545")?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers() // Adds gas estimation, nonce management, and chain ID fetching
        .wallet(wallet)
        .on_http(rpc_url);

    // Deploy the contract with an initial value of 1
    let initial_value = U256::from(1);
    let contract = SampleContract::deploy(&provider, initial_value).await?;
    println!("📦 Contract deployed with initial value: {}", initial_value);

    // Retrieve the initial value from the contract
    let current_value = contract.getValue().call().await?.currentValue;
    println!("🔍 Initial value retrieved from contract: {}", current_value);

    // Set the contract value to 2
    let new_value = U256::from(2);
    let tx_builder = contract.setValue(new_value).send().await?;
    let pending_tx = tx_builder.register().await?;
    let tx_hash = pending_tx.await?;
    println!("🔄 Transaction sent to set new value. Transaction hash: {:#x}", tx_hash);

    // Get the transaction receipt and decode logs
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("Transaction receipt not found");
    println!("🧾 Transaction receipt obtained. Receipt hash: {:#x}", receipt.transaction_hash);

    // Decode and handle the ValueChanged event from the transaction receipt
    for log in receipt.inner.logs() {
        // Decode the log and access the `data` field
        let log = SampleContract::SampleContractEvents::decode_log(log.as_ref(), true)?;
        if let SampleContract::SampleContractEvents::ValueChanged(event) = log.data {
            println!("⚡️ Event: ValueChanged - newValue: {}", event.newValue);
        }
    }

    // Verify that the updated value matches the expected result
    let updated_value = contract.getValue().call().await?.currentValue;
    println!("🔍 Updated value retrieved from contract: {}", updated_value);

    // Retrieve the initial contract and signer balances
    let contract_balance = contract.getBalance().call().await?.balance;
    println!("🔍 Initial contract balance: {} Ξ", format_ether(contract_balance));
    let signer_balance = provider.get_balance(signer_address).await?;
    println!("🔍 Initial signer balance: {} Ξ", format_ether(signer_balance));

    // Deposit Ether to the contract (half of the signer's balance)
    let deposit_amount = signer_balance.div(U256::from(2));
    let tx_builder = contract.deposit().value(deposit_amount).send().await?;
    let pending_tx = tx_builder.register().await?;
    let tx_hash = pending_tx.await?;
    println!(
        "🔄 Transaction sent to deposit Ether. Transaction hash: {:#x}",
        tx_hash
    );

    // Get the transaction receipt and decode logs
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("Transaction receipt not found");
    println!("🧾 Transaction receipt obtained. Receipt hash: {:#x}", receipt.transaction_hash);

    // Decode and handle the ValueChanged event from the transaction receipt
    for log in receipt.inner.logs() {
        // Decode the log and access the `data` field
        let log = SampleContract::SampleContractEvents::decode_log(log.as_ref(), true)?;
        if let SampleContract::SampleContractEvents::EtherReceived(event) = log.data {
            println!("⚡️ Event: EtherReceived - sender: {}; amount: {}", event.sender, event.amount);
        }
    }

    // Retrieve the intermediate contract and signer balances
    let contract_balance = contract.getBalance().call().await?.balance;
    println!("🔍 Contract balance after deposit: {} Ξ", format_ether(contract_balance));
    let signer_balance = provider.get_balance(signer_address).await?;
    println!("🔍 Signer balance after deposit: {} Ξ", format_ether(signer_balance));

    // Execute a call to revertWithError to trigger a `revert SampleError("failed")` error
    match contract.revertWithError().call().await {
        Ok(_) => {
            // Handle successful call if necessary
        }
        Err(Error::TransportError(transport_error)) => {
            // Attempt to decode the error into SampleContractErrors directly
            match transport_error
                .as_error_resp()
                .and_then(|error| error.as_decoded_error::<SampleContractErrors>(true))
            {
                Some(SampleContractErrors::SampleError(sample_error)) => {
                    println!("⚠️ Call reverted with SampleError: {:?}", sample_error.message);
                },
                // You can add other SampleContractErrors variants here if needed.
                _ => {
                    println!("⚠️ Call reverted with unexpected error: {:?}", transport_error);
                }
            }
        }
        Err(error) => {
            // Handle other error variants if necessary
            println!("⚠️ Call reverted with unexpected error: {:?}", error);
        }
    }

    Ok(())
}

