use std::sync::Arc;
use alloy_primitives::{address, Address};
use alloy_provider::{Provider, ProviderBuilder, ReqwestProvider};
use alloy_sol_macro::sol;
use eyre::Result;
use tokio::main;
use url::Url;
use alloy_contract::SolCallBuilder;

sol! {
    #[sol(rpc, bytecode = "6942")]
    contract MinimalInteraction {
        uint256 public value;

        event ValueChanged(uint256 newValue);
        event EtherReceived(address sender, uint256 amount);

        constructor(uint256 _initialValue) {
            value = _initialValue;
        }

        function setValue(uint256 _value) external {
            value = _value;
            emit ValueChanged(_value);
        }

        function getValue() external view returns (uint256) {
            return value;
        }

        receive() external payable {
            emit EtherReceived(msg.sender, msg.value);
        }

        fallback() external payable {
            emit EtherReceived(msg.sender, msg.value);
        }

        function withdraw() external {
            payable(msg.sender).transfer(address(this).balance);
        }

        function getBalance() external view returns (uint256) {
            return address(this).balance;
        }
    }
}

#[main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let rpc_url: Url = "http://192.168.0.188:8545".parse()?;
    let provider: ReqwestProvider = ProviderBuilder::new().on_http(rpc_url);
    let provider = Arc::new(provider);

    MinimalInteraction::deploy()

    //let usdc: Address = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse()?;
    //let token = MinimalInteraction::new(usdc, Arc::clone(&provider));

    // match token.name().call().await {
    //     Ok(name) => println!("Token name: {}", name.name),
    //     Err(e) => eprintln!("Error fetching token name: {:?}", e),
    // }

    Ok(())
}

