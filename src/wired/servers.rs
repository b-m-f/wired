use super::clients::ClientConfig;
use super::crypto::{derive_base64_public_key_from_base64_private_key, get_private_key};
use super::network::NetworkConfig;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct ServerConfig {
    pub endpoint: String,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub persistent_keepalive: Option<u16>,
    pub public_key: String,
    dns: Option<String>,
    private_key: String,
    pub output: String,
    pub name: String,
}

impl ServerConfig {
    pub fn generate_nix(&self, clients: &Vec<ClientConfig>, pre_shared_key: &String) -> String {
        return "test".to_string();
    }

    pub fn generate_conf(&self, clients: &Vec<ClientConfig>, pre_shared_key: &String) -> String {
        let mut server_section = format!(
            "[Interface]\n\
    Address = {}\n\
    ListenPort = {}\n\
    PrivateKey = {}\n",
            self.ip, self.port, self.private_key
        );
        match &self.dns {
            Some(dns) => {
                server_section = format!(
                    "{}\
             DNS = {}",
                    server_section, dns,
                )
            }
            None => (),
        }

        let mut peers = "".to_string();
        for peer in &clients[..] {
            peers = format!(
                "{}\n\
[Peer]\n\
# friendly_name = {}\n
AllowedIPs = {}\n\
PublicKey = {}\n\
PresharedKey = {}",
                peers, peer.name, peer.ip, peer.public_key, pre_shared_key
            );
            peers = format!("{}\n", peers)
        }
        format!("{}\n{}\n", server_section, peers)
    }
    // TODO: add nix config generation
}

pub fn parse_server_configs(
    servers: toml::value::Table,
    network: &NetworkConfig,
) -> Vec<ServerConfig> {
    // TODO: test parsing error
    let mut configs: Vec<ServerConfig> = Vec::new();
    let mut ips: Vec<Ipv4Addr> = [].to_vec();

    for (key, value) in servers.iter() {
        //  TODO: remove path to config
        let name = key;
        let private_key = match value.get("privatekey") {
            // TODO: make parse and replace easier, pull into function
            Some(key) => key.to_string().replace("\"", ""),
            None => get_private_key(),
        };

        let server_config: ServerConfig = ServerConfig {
            name: name.to_string(),
            endpoint: match value.get("endpoint") {
                // TODO: validate that its a correct host
                Some(endpoint) => endpoint.to_string().replace("\"", ""),
                None => panic!("Server configurations need an Endpoint"),
            },
            dns: match value.get("dns") {
                Some(dns) => Some(dns.to_string().replace("\"", "")),
                None => None,
            },
            ip: match value.get("ip") {
                Some(ip_from_config) => {
                    let ip: Ipv4Addr = match ip_from_config.to_string().replace("\"", "").parse() {
                        Ok(ip) => ip,
                        Err(e) => panic!(
                            "Error when trying to parse IP {} for server {}: {}",
                            ip_from_config, key, e
                        ),
                    };
                    if network.cidrv4.contains(&ip) {
                        ips.push(ip);
                        ip
                    } else {
                        panic!("Error when trying to parse IP for server {}: IP is not inside provided network range", key )
                    }
                }
                None => panic!("Server configurations need an IP"),
            },
            persistent_keepalive: match value.get("persistent_keepalive") {
                Some(ka) => match ka.to_string().parse() {
                    Ok(ka) => Some(ka),
                    Err(e) => panic!("Could not read servers keepalive: {}", e),
                },
                None => None,
            },
            port: match value.get("listenport") {
                Some(port) => match port.to_string().parse() {
                    Ok(port) => port,
                    Err(e) => panic!("Could not read servers port: {}", e),
                },
                None => panic!(
                    "Servers need a port configured so that WireGuard can listen for connections"
                ),
            },
            private_key: private_key.to_string(),
            public_key: derive_base64_public_key_from_base64_private_key(&private_key),
            output: match value.get("output") {
                Some(r#type) => r#type.to_string().replace("\"", ""),
                None => "conf".to_string(),
            },
        };

        configs.push(server_config);
    }
    configs
}
