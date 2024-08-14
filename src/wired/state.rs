use super::{clients::ClientConfig, network::NetworkConfig, servers::ServerConfig};

pub fn create_statefile(
    network: &NetworkConfig,
    servers: &Vec<ServerConfig>,
    client: &Vec<ClientConfig>,
) -> String {
    let network_toml = toml::to_string(network).unwrap();
    // println!("{network_toml}");
    return "".to_string();
}
