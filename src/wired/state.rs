use super::{clients::ClientConfig, network::NetworkConfig, servers::ServerConfig};

pub fn create_statefile(
    network: &NetworkConfig,
    servers: &Vec<ServerConfig>,
    clients: &Vec<ClientConfig>,
) -> String {
    let network_toml = toml::to_string(network).unwrap();
    let network = format!("[network]\n{network_toml}");

    let mut server_string = "[servers]\n".to_string();
    for server in servers {
        let server_toml = toml::to_string(server).unwrap();
        server_string.push_str(format!("[Servers.{}]\n", server.name).as_str());
        server_string.push_str(&server_toml);
    }
    let mut client_string = "[clients]\n".to_string();
    for client in clients {
        let client_toml = toml::to_string(client).unwrap();
        client_string.push_str(format!("[Clients.{}]\n", client.name).as_str());
        client_string.push_str(&client_toml);
    }

    return format!("{network}\n{server_string}\n{client_string}");
}
