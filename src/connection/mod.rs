use std::env;
use solana_rpc_client::rpc_client::RpcClient;

pub enum ConnectionNetwork {
    MAINNET,
    DEVNET,
    TESTNET
}

pub struct Connection {
    pub network: ConnectionNetwork,
}

impl Connection {
    pub fn new(&self) -> Self {}

    fn solana_url() -> String {
        // Retrieve the NETWORK environment variable
        let network = env::var("NETWORK").unwrap_or_else(|_| "devnet".to_string());

        // Define the URLs
        let solana_mainnet = env::var("SOLANA_MAINNET").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let solana_devnet = env::var("SOLANA_DEVNET").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        let solana_testnet = env::var("SOLANA_TESTNET").unwrap_or_else(|_| "https://api.testnet.solana.com".to_string());

        // Match the NETWORK variable and return the corresponding URL
        match network.as_str() {
            "mainnet" => solana_mainnet,
            "devnet" => solana_devnet,
            "testnet" => solana_testnet,
            _ => {
                eprintln!("Unknown network: {}", network);
                std::process::exit(1);
            }
        }
    }

    pub fn connection(&self) -> RpcClient {}
}

fn get_network() -> ConnectionNetwork {
    let network = env::var("NETWORK").unwrap_or_else(|_| "devnet".to_string());

    match network.as_str() {
        "mainnet" => ConnectionNetwork::MAINNET,
        "devnet" => ConnectionNetwork::DEVNET,
        "testnet" => ConnectionNetwork::TESTNET,
        _ => {
            eprintln!("Unknown network: {}", network);
            std::process::exit(1);
        }
    }
}

