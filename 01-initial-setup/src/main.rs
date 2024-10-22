use std::ops::Div;
use alloy_network::EthereumWallet;
use alloy_primitives::{Log, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_macro::sol;
use alloy_sol_types::SolEventInterface;
use eyre::Result;
use tokio::main;
use tracing_subscriber;
use url::Url;

sol! {
    // bytecode via `solc SampleContract.sol --bin --via-ir --optimize --optimize-runs 1`
    #[sol(rpc, bytecode = "608034604d57601f6101ec38819003918201601f19168301916001600160401b03831184841017605157808492602094604052833981010312604d57515f5560405161018690816100668239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c90816312065fe0146101385750806320965255146100e15780633ccfd60b146100fd5780633fa4f245146100e1578063552410771461009e5763d0e30db01461005e575f80fd5b5f36600319011261009a577f1e57e3bb474320be3d2c77138f75b7c3941292d647f5f9634e33a8e94e0e069b60408051338152346020820152a1005b5f80fd5b3461009a57602036600319011261009a577f93fe6d397c74fdf1402a8b72e47b68512f0510d7b98a4bc4cbdf6ac7108b3c596020600435805f55604051908152a1005b3461009a575f36600319011261009a5760205f54604051908152f35b3461009a575f36600319011261009a575f8080804781811561012f575b3390f11561012457005b6040513d5f823e3d90fd5b506108fc61011a565b3461009a575f36600319011261009a57602090478152f3fea2646970667358221220297537b9d5725826ae82c1a2623d5575f7d002c81e438dce588287e74603903c64736f6c634300081b0033")]
    contract SampleContract {
        uint256 public value;

        event ValueChanged(uint256 newValue);
        event EtherReceived(address sender, uint256 amount);

        constructor(uint256 _initialValue);

        function setValue(uint256 _value) external;
        function getValue() external view returns (uint256 currentValue);
        function deposit() external payable;
        function withdraw() external;
        function getBalance() external view returns (uint256 balance);
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
    println!("Contract deployed with initial value: {}", initial_value);

    // Retrieve the initial value from the contract
    let current_value = contract.getValue().call().await?.currentValue;
    println!("Initial value retrieved from contract: {}", current_value);

    // Set the contract value to 2
    let new_value = U256::from(2);
    let tx_builder = contract.setValue(new_value).send().await?;
    let pending_tx = tx_builder.register().await?;
    let tx_hash = pending_tx.await?;
    println!("Transaction sent to set new value. Transaction hash: {:#x}", tx_hash);

    // Get the transaction receipt and decode logs
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("Transaction receipt not found");
    println!("Transaction receipt obtained. Receipt hash: {:#x}", receipt.transaction_hash);

    // Decode and handle events from the transaction receipt
    for log in receipt.inner.logs() {
        let decoded_log: Log<SampleContract::SampleContractEvents> =
            SampleContract::SampleContractEvents::decode_log(log.as_ref(), true)?;
        match decoded_log.data {
            SampleContract::SampleContractEvents::ValueChanged(event) => {
                println!("Event: ValueChanged - newValue: {}", event.newValue);
            }
            SampleContract::SampleContractEvents::EtherReceived(event) => {
                println!(
                    "Event: EtherReceived - sender: {}; amount: {}",
                    event.sender, event.amount
                );
            }
        }
    }

    // Verify that the updated value matches the expected result
    let updated_value = contract.getValue().call().await?.currentValue;
    println!("Updated value retrieved from contract: {}", updated_value);

    // Retrieve the initial contract and signer balances
    let contract_balance = contract.getBalance().call().await?.balance;
    println!("Initial contract balance: {}", contract_balance);
    let signer_balance = provider.get_balance(signer_address).await?;
    println!("Initial signer balance: {}", signer_balance);

    // Deposit Ether to the contract (half of the signer's balance)
    let deposit_amount = signer_balance.div(U256::from(2));
    let tx_builder = contract.deposit().value(deposit_amount).send().await?;
    let pending_tx = tx_builder.register().await?;
    let tx_hash = pending_tx.await?;
    println!(
        "Transaction sent to deposit Ether. Transaction hash: {:#x}",
        tx_hash
    );

    // Get the transaction receipt and decode logs
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("Transaction receipt not found");
    println!("Transaction receipt obtained. Receipt hash: {:#x}", receipt.transaction_hash);

    // Decode and handle events from the transaction receipt
    for log in receipt.inner.logs() {
        let decoded_log: Log<SampleContract::SampleContractEvents> =
            SampleContract::SampleContractEvents::decode_log(log.as_ref(), true)?;
        match decoded_log.data {
            SampleContract::SampleContractEvents::ValueChanged(event) => {
                println!("Event: ValueChanged - newValue: {}", event.newValue);
            }
            SampleContract::SampleContractEvents::EtherReceived(event) => {
                println!(
                    "Event: EtherReceived - sender: {}; amount: {}",
                    event.sender, event.amount
                );
            }
        }
    }

    // Retrieve the intermediate contract and signer balances
    let contract_balance = contract.getBalance().call().await?.balance;
    println!("Contract balance after deposit: {}", contract_balance);
    let signer_balance = provider.get_balance(signer_address).await?;
    println!("Signer balance after deposit: {}", signer_balance);

    // Withdraw Ether from the contract
    let tx_builder = contract.withdraw().send().await?;
    let pending_tx = tx_builder.register().await?;
    let tx_hash = pending_tx.await?;
    println!(
        "Transaction sent to withdraw Ether. Transaction hash: {:#x}",
        tx_hash
    );

    // Get the transaction receipt and decode logs
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("Transaction receipt not found");
    println!("Transaction receipt obtained. Receipt hash: {:#x}", receipt.transaction_hash);

    // Decode and handle events from the transaction receipt
    for log in receipt.inner.logs() {
        let decoded_log: Log<SampleContract::SampleContractEvents> =
            SampleContract::SampleContractEvents::decode_log(log.as_ref(), true)?;
        match decoded_log.data {
            SampleContract::SampleContractEvents::ValueChanged(event) => {
                println!("Event: ValueChanged - newValue: {}", event.newValue);
            }
            SampleContract::SampleContractEvents::EtherReceived(event) => {
                println!(
                    "Event: EtherReceived - sender: {}; amount: {}",
                    event.sender, event.amount
                );
            }
        }
    }

    // Retrieve the final contract and signer balances
    let contract_balance = contract.getBalance().call().await?.balance;
    println!("Final contract balance after withdrawal: {}", contract_balance);
    let signer_balance = provider.get_balance(signer_address).await?;
    println!("Final signer balance after withdrawal: {}", signer_balance);

    Ok(())
}
