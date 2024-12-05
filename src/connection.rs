use solana_rpc_client::rpc_client::RpcClient;
use std::env;

#[derive(Debug, PartialEq)]
pub enum ConnectionNetwork {
    MAINNET,
    DEVNET,
    TESTNET,
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

    fn default_url(&self) -> String {
        match self {
            ConnectionNetwork::MAINNET => "https://api.mainnet-beta.solana.com".to_string(),
            ConnectionNetwork::DEVNET => "https://api.devnet.solana.com".to_string(),
            ConnectionNetwork::TESTNET => "https://api.testnet.solana.com".to_string(),
        }
    }
}

pub struct Connection {
    pub network: ConnectionNetwork,
}

impl Connection {
    pub fn new() -> Self {
        let network_env = env::var("NETWORK").unwrap_or_else(|_| "devnet".to_string());
        let network = ConnectionNetwork::from_str(&network_env).expect("invalid network");
        Self { network }
    }

    pub fn connection(&self) -> RpcClient {
        let url = self.solana_url();
        RpcClient::new(url)
    }

    fn solana_url(&self) -> String {
        // Define the URLs
        let solana_mainnet =
            env::var("SOLANA_MAINNET").unwrap_or_else(|_| self.network.default_url());
        let solana_devnet =
            env::var("SOLANA_DEVNET").unwrap_or_else(|_| self.network.default_url());
        let solana_testnet =
            env::var("SOLANA_TESTNET").unwrap_or_else(|_| self.network.default_url());

        // Match the NETWORK variable and return the corresponding URL
        match self.network {
            ConnectionNetwork::MAINNET => solana_mainnet,
            ConnectionNetwork::DEVNET => solana_devnet,
            ConnectionNetwork::TESTNET => solana_testnet,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_network_from_str() {
        assert_eq!(
            ConnectionNetwork::from_str("mainnet"),
            Some(ConnectionNetwork::MAINNET)
        );
        assert_eq!(
            ConnectionNetwork::from_str("devnet"),
            Some(ConnectionNetwork::DEVNET)
        );
        assert_eq!(
            ConnectionNetwork::from_str("testnet"),
            Some(ConnectionNetwork::TESTNET)
        );
        assert_eq!(
            ConnectionNetwork::from_str("MAINNET"),
            Some(ConnectionNetwork::MAINNET)
        );
        assert_eq!(ConnectionNetwork::from_str("unknown"), None);
    }

    #[test]
    fn test_connection_network_default_to_devnet() {
        env::remove_var("NETWORK");
        let connection = Connection::new();
        assert_eq!(connection.network, ConnectionNetwork::DEVNET);
    }

    #[test]
    fn test_connection_network_from_env() {
        env::set_var("NETWORK", "mainnet");
        let connection = Connection::new();
        assert_eq!(connection.network, ConnectionNetwork::MAINNET);

        env::set_var("NETWORK", "testnet");
        let connection = Connection::new();
        assert_eq!(connection.network, ConnectionNetwork::TESTNET);
    }

    #[test]
    #[should_panic(expected = "invalid network")]
    fn test_connection_invalid_network_panic() {
        env::set_var("NETWORK", "invalid");
        Connection::new();
    }

    #[test]
    fn test_solana_url_resolution() {
        env::set_var("NETWORK", "mainnet");
        env::set_var("SOLANA_MAINNET", "http://custom.mainnet.url");
        let connection = Connection::new();
        assert_eq!(connection.solana_url(), "http://custom.mainnet.url");

        env::set_var("NETWORK", "devnet");
        env::set_var("SOLANA_DEVNET", "http://custom.devnet.url");
        let connection = Connection::new();
        assert_eq!(connection.solana_url(), "http://custom.devnet.url");

        env::set_var("NETWORK", "testnet");
        env::set_var("SOLANA_TESTNET", "http://custom.testnet.url");
        let connection = Connection::new();
        assert_eq!(connection.solana_url(), "http://custom.testnet.url");
    }

    #[test]
    fn test_solana_url_default_values() {
        env::set_var("NETWORK", "mainnet");
        env::remove_var("SOLANA_MAINNET");
        let connection = Connection::new();
        assert_eq!(
            connection.solana_url(),
            "https://api.mainnet-beta.solana.com"
        );

        env::set_var("NETWORK", "devnet");
        env::remove_var("SOLANA_DEVNET");
        let connection = Connection::new();
        assert_eq!(connection.solana_url(), "https://api.devnet.solana.com");

        env::set_var("NETWORK", "testnet");
        env::remove_var("SOLANA_TESTNET");
        let connection = Connection::new();
        assert_eq!(connection.solana_url(), "https://api.testnet.solana.com");
    }
}
