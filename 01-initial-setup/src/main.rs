use std::sync::Arc;
use alloy_primitives::{address, Address, U256};
use alloy_provider::{Provider, ProviderBuilder, ReqwestProvider, WalletProvider};
// use alloy_provider::{
//     layers::AnvilProvider, Provider, ProviderBuilder, RootProvider, WalletProvider,
// };
use alloy_sol_macro::sol;
use eyre::Result;
use tokio::main;
use url::Url;
use alloy_contract::SolCallBuilder;
use alloy_network::{EthereumWallet, NetworkWallet};
use alloy_rpc_types::TransactionRequest;
use alloy_signer_local::PrivateKeySigner;

sol! {
    #[sol(rpc, bytecode = "608034604d57601f6101ee38819003918201601f19168301916001600160401b03831184841017605157808492602094604052833981010312604d57515f5560405161018890816100668239f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610028575b5036156100205761001e610121565b005b61001e610121565b5f3560e01c90816312065fe0146101095750806320965255146100b25780633ccfd60b146100ce5780633fa4f245146100b25763552410771461006b575f61000f565b346100ae5760203660031901126100ae577f93fe6d397c74fdf1402a8b72e47b68512f0510d7b98a4bc4cbdf6ac7108b3c596020600435805f55604051908152a1005b5f80fd5b346100ae575f3660031901126100ae5760205f54604051908152f35b346100ae575f3660031901126100ae575f80808047818115610100575b3390f1156100f557005b6040513d5f823e3d90fd5b506108fc6100eb565b346100ae575f3660031901126100ae57602090478152f35b7f1e57e3bb474320be3d2c77138f75b7c3941292d647f5f9634e33a8e94e0e069b60408051338152346020820152a156fea26469706673582212207beab8e7bc293868bcf7daa2086c572cbee13a922660964538f46ebc33117aca64736f6c634300081b0033")]
    contract MinimalInteraction {
        uint256 public value;

        event ValueChanged(uint256 newValue);
        event EtherReceived(address sender, uint256 amount);

        constructor(uint256 _initialValue);

        function setValue(uint256 _value) external;

        function getValue() external view returns (uint256 currentValue);

        receive() external payable;

        fallback() external payable;

        function withdraw() external;

        function getBalance() external view returns (uint256 balance);
    }
}

#[main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let signer: PrivateKeySigner = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
    let wallet = EthereumWallet::from(signer);

    //let rpc_url: Url = "http://192.168.0.188:8545".parse()?;
    let rpc_url: Url = "http://127.0.0.1:8545".parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()// gas estimation, nonce management, and chain-id fetching
        .wallet(wallet)
        .on_http(rpc_url);

    let provider = Arc::new(provider);

    // use deploy_builder if manually setting gas/nonce
    let contract =
        MinimalInteraction::deploy(Arc::clone(&provider), U256::from(1)).await?;

    let value: U256 = contract.getValue().call().await?.currentValue;
    println!("value: {}", value);


    let builder = contract.setValue(U256::from(2)).send().await?;
    let pending_tx = builder.register().await?;
    let tx_hash = pending_tx.await?;
    println!("tx_hash: {}", tx_hash);

    let receipt =
        provider.get_transaction_receipt(tx_hash).await?.expect("Transaction receipt not found");

    let receipt_hash = receipt.transaction_hash;

    println!("receipt_hash: {}", receipt_hash);

    let value: U256 = contract.getValue().call().await?.currentValue;
    println!("value: {}", value);

    Ok(())
}

