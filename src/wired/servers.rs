use super::clients::ClientConfig;
use super::crypto::{
    derive_base64_public_key_from_base64_private_key, get_private_key_from_file_or_generate,
};
use super::global::GlobalConfig;
use std::net::IpAddr;
use std::path::Path;

#[derive(Debug)]
pub struct ServerConfig {
    pub endpoint: String,
    pub ip: IpAddr,
    pub port: u16,
    pub persistent_keepalive: Option<u16>,
    pub public_key: String,
    pub path_to_config: String,
    dns: Option<String>,
    private_key: String,
}

impl ServerConfig {
    pub fn generate_string(&self, clients: &Vec<ClientConfig>, pre_shared_key: &String) -> String {
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
AllowedIPs = {}\n\
PublicKey = {}\n\
PresharedKey = {}",
                peers, peer.ip, peer.public_key, pre_shared_key
            );
            peers = format!("{}\n", peers)
        }
        format!("{}\n{}\n", server_section, peers)
    }
}

pub fn parse_server_configs(
    servers: toml::value::Table,
    global: &GlobalConfig,
) -> Vec<ServerConfig> {
    // TODO: test parsing error
    let mut configs: Vec<ServerConfig> = Vec::new();
    let mut ips: Vec<IpAddr> = [].to_vec();

    for (key, value) in servers.iter() {
        let path_string = format!("./{}/{}.conf", global.name, key);
        let path = Path::new(&path_string);
        let private_key = &get_private_key_from_file_or_generate(&path, global.rotate_keys);

        let server_config: ServerConfig = ServerConfig {
            path_to_config: path.display().to_string(),
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
                    let ip: IpAddr = match ip_from_config.to_string().replace("\"", "").parse() {
                        Ok(ip) => ip,
                        Err(e) => panic!(
                            "Error when trying to parse IP {} for server {}: {}",
                            ip_from_config, key, e
                        ),
                    };
                    if global.cidr.contains(&ip) {
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
            port: match value.get("port") {
                Some(port) => match port.to_string().parse() {
                    Ok(port) => port,
                    Err(e) => panic!("Could not read servers port: {}", e),
                },
                None => panic!(
                    "Servers need a port conigured so that WireGuard can listen for connections"
                ),
            },
            private_key: private_key.to_string(),
            public_key: derive_base64_public_key_from_base64_private_key(&private_key),
        };

        configs.push(server_config);
    }
    configs
}
