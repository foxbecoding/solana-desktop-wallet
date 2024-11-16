use std::env;
use solana_rpc_client::rpc_client::RpcClient;

pub enum ConnectionNetwork {
    MAINNET,
    DEVNET,
    TESTNET
}

impl ConnectionNetwork {
    fn from_str(network: &str) -> Option<Self> {
        match network.to_lowercase().as_str() {
            "mainnet" => Some(ConnectionNetwork::MAINNET),
            "devnet" => Some(ConnectionNetwork::DEVNET),
            "testnet" => Some(ConnectionNetwork::TESTNET),
            _ => None,
        }
    }
}

pub struct Connection {
    pub network: ConnectionNetwork,
}

impl Connection {
    pub fn new(&self) -> Self {
        let network_env = env::var("NETWORK").unwrap_or_else(|_| "devnet".to_string());
        let network = ConnectionNetwork::from_str(&network_env).expect("invalid network");
        Self { network }
    }

    fn solana_url(&self) -> String {
        // Define the URLs
        let solana_mainnet = env::var("SOLANA_MAINNET").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let solana_devnet = env::var("SOLANA_DEVNET").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        let solana_testnet = env::var("SOLANA_TESTNET").unwrap_or_else(|_| "https://api.testnet.solana.com".to_string());

        // Match the NETWORK variable and return the corresponding URL
        match self.network {
            ConnectionNetwork::MAINNET => solana_mainnet,
            ConnectionNetwork::DEVNET => solana_devnet,
            ConnectionNetwork::TESTNET => solana_testnet,
        }
    }

    pub fn connection(&self) -> RpcClient {}
}