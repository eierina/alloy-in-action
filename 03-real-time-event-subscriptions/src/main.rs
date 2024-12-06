use std::{io::stdin, path::Path};
use eyre::Result;
use futures::StreamExt;
use url::Url;
use alloy_network::EthereumWallet;
use alloy_primitives::{Address, B256, U256, utils::Unit};
use alloy_provider::{Provider, ProviderBuilder, WalletProvider, WsConnect};
use alloy_rpc_types::{BlockNumberOrTag, Filter};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_macro::sol;
use alloy_sol_types::{SolEventInterface, SolEvent};
use crate::SampleContract::{EtherReceived, EtherWithdrawn, SampleContractEvents, ValueChanged};

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

    // Initialize signers and wallet
    let private_key = std::env::var("ANVIL_PRIVATE_KEY")?;
    let secondary_private_key = std::env::var("ANVIL_SECONDARY_PRIVATE_KEY")?;
    let signer: PrivateKeySigner = private_key.parse()?;
    let secondary_signer: PrivateKeySigner = secondary_private_key.parse()?;
    let signer_address = signer.address();
    let secondary_signer_address = secondary_signer.address();
    let wallet: EthereumWallet = EthereumWallet::from(signer);

    // Set up provider using WebSocket
    let ws_url = std::env::var("ANVIL_WS_URL")?;
    let ws_url = Url::parse(&ws_url)?;
    let mut provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_ws(WsConnect::new(ws_url)).await?;

    // Deploy the contract with an initial value of 1
    let initial_value = U256::from(1);
    let mut contract = SampleContract::deploy(provider.clone(), initial_value).await?;
    let contract_address: Address = *contract.address();
    println!(
        "📦 Contract deployed at address {} with initial value: {}",
        &contract_address, initial_value
    );

    // Set up event filters
    let address_filter: Vec<B256> = vec![
        B256::left_padding_from(&signer_address.0.0),
        B256::left_padding_from(&secondary_signer_address.0.0)
    ];

    // Create a filter for the ValueChanged event starting from the latest block
    let value_changed_filter = contract
        .ValueChanged_filter()
        .topic1(address_filter.clone())
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to the ValueChanged event logs
    let value_changed_subscription = value_changed_filter.subscribe().await?;
    println!("📡 Subscribed to ValueChanged events.");

    // Convert the subscription into a stream for processing
    let mut value_changed_stream = value_changed_subscription.into_stream();

    // Spawn a task to handle incoming ValueChanged events
    tokio::spawn(async move {
        println!("👂 Listening for ValueChanged events...");
        while let Some(result) = value_changed_stream.next().await {
            match result {
                Ok((event, log)) => {
                    // Print details of the ValueChanged event
                    println!(
                        "⚡️ |ValueChanged| - updater: {}, oldValue: {}, newValue: {} [{}]",
                        event.updater, event.oldValue, event.newValue, log.address()
                    );
                }
                Err(e) => {
                    eprintln!("⚠️ Error processing event: {:?}", e)
                    // Handle error ...
                }
            }
        }
    });

    // Create a combined filter for multiple events
    let events_filter = Filter::new()
        .address(contract_address)
        .topic1(address_filter.clone())
        .event_signature(vec![
            ValueChanged::SIGNATURE_HASH,
            EtherReceived::SIGNATURE_HASH,
            EtherWithdrawn::SIGNATURE_HASH
        ])
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to the combined events filter
    let events_subscription = provider.subscribe_logs(&events_filter).await?;
    println!("📡 Subscribed to combined events.");

    // Convert the subscription into a stream for processing
    let mut events_stream = events_subscription.into_stream();

    // Spawn a task to listen and decode all contract events into their specific types
    tokio::spawn(async move {
        println!("👂 Listening for events...");
        while let Some(log) = events_stream.next().await {
            // Decode the log into SampleContractEvents enum
            match SampleContractEvents::decode_log(log.as_ref(), true) {
                Ok(event) => {
                    match event.data {
                        SampleContractEvents::ValueChanged(e) => {
                            println!(
                                "⚡️ ValueChanged   - updater: {}, oldValue: {}, newValue: {} [{}] ",
                                e.updater, e.oldValue, e.newValue, log.address()
                            );
                        }
                        SampleContractEvents::EtherReceived(e) => {
                            println!(
                                "⚡️ EtherReceived  - sender: {}, amount: {}, newBalance: {} [{}]",
                                e.sender, e.amount, e.newBalance, log.address()
                            );
                        }
                        SampleContractEvents::EtherWithdrawn(e) => {
                            println!(
                                "⚡️ EtherWithdrawn - recipient: {}, amount: {}, remainingBalance: {} [{}]",
                                e.recipient, e.amount, e.remainingBalance, log.address()
                            );
                        }
                    }
                }
                Err(_) => {
                    eprintln!("⚠️ Unknown event received.")
                    // Handle error ...
                }
            }
        }
    });

    // Send Transactions

    for i in 0..2 {
        // 1. Set the contract value to (i + 1) to trigger the ValueChanged event
        println!("🔄 Sending transaction to set new value.");
        let new_value = U256::from(i + 2);
        let _ = contract.setValue(new_value).send().await?;

        // 2. Deposit 1 Pwei to the contract
        println!("🔄 Sending transaction to deposit Ether.");
        let _ = contract.deposit().value(Unit::PWEI.wei()).send().await?;

        // 3. Withdraw balance from the contract
        println!("🔄 Sending transaction to withdraw Ether.");
        let _ = contract.withdraw().send().await?;

        // Change the signer
        provider.wallet_mut().register_default_signer(secondary_signer.clone());
        contract = SampleContract::new(contract_address, provider.clone());
    }

    // Keep the main function alive until all expected events are processed
    println!("⏳ All transactions sent. Waiting for events. Press Enter to exit.");
    stdin().read_line(&mut String::new())?;

    Ok(())
}
