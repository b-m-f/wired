use core::panic;

use ipnet::Ipv4Net;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::str::FromStr;

use super::clients::ClientConfig;
use super::crypto::{
    derive_base64_public_key_from_base64_private_key, get_preshared_key, get_private_key,
};
use super::network::NetworkConfig;
use super::servers::ServerConfig;

#[derive(Debug, Deserialize)]
pub struct NetworkConfigFromFile {
    pub cidrv4: String,
    pub name: String,
    pub presharedkey: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub network: NetworkConfigFromFile,
    pub servers: toml::value::Table,
    pub clients: toml::value::Table,
}
pub fn parse_config(config: String) -> Config {
    match toml::from_str(&config) {
        Ok(config) => config,
        Err(e) => panic!("Error when parsing configuration: {}", e),
    }
}

pub fn parse_network(config: &Config) -> NetworkConfig {
    // TODO: test parsing errors
    let cidrv4: Ipv4Net = config.network.cidrv4.parse().unwrap();
    let name: &String = &config.network.name;
    NetworkConfig {
        cidrv4,
        // TODO: get name from config file
        name: name.to_string(),
        // TODO: make sure this is caught in parsing
        preshared_key: match &config.network.presharedkey {
            Some(psk) => psk.to_string(),
            None => get_preshared_key(),
        },
        // TODO: Parse and set web as default
        // TODO: Make own doc file for network types
        r#type: "web".to_string(),
    }
}

pub fn parse_servers(config: &Config) -> Vec<ServerConfig> {
    let servers = &config.servers;
    let network = parse_network(&config);
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

pub fn parse_clients(config: &Config) -> Vec<ClientConfig> {
    let clients = &config.clients;
    let servers = parse_servers(&config);
    let network = parse_network(&config);

    // TODO: test parsing error
    let mut configs: Vec<ClientConfig> = Vec::new();

    let mut configs_without_ip: Vec<ClientConfig> = Vec::new();

    for (client_name, client) in clients.iter() {
        if client.is_table() {
            let table = match client.as_table() {
                Some(table) => table,
                None => panic!(
                    "Error when parsing client {}. Client is not a proper TOML table",
                    client_name
                ),
            };
            for (field_key, field_value) in table.iter() {
                let name: String = client_name.to_string();
                let mut private_key: String = "".to_string();
                let mut ip: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
                let mut dns: Option<String> = None;
                let mut output: String = "conf".to_string();
                let mut postpone_config_generation_until_all_defined_ips_are_known = false;
                println!("{}={}", field_key, field_value);
                match field_key.as_str() {
                    "privatekey" => {
                        private_key = match client.get(field_key) {
                            Some(key) => key.to_string().replace("\"", ""),
                            None => get_private_key(),
                        }
                    }
                    "ip" => {
                        ip = match client.get(field_key) {
                            Some(ip) => {
                                match Ipv4Addr::from_str(&ip.to_string().replace("\"", "")) {
                                    Ok(ip) => ip,
                                    Err(e) => {
                                        panic!("Error when parsing IP of client {}: {}", &name, e)
                                    }
                                }
                            }
                            None => {
                                postpone_config_generation_until_all_defined_ips_are_known = true;
                                Ipv4Addr::new(0, 0, 0, 0)
                            }
                        }
                    }
                    "dns" => {
                        dns = match client.get(field_key) {
                            Some(dns) => Some(dns.to_string().replace("\"", "")),
                            None => None,
                        }
                    }
                    "output" => {
                        output = match client.get(field_key) {
                            Some(output) => output.to_string().replace("\"", ""),
                            None => output,
                        }
                    }
                    _ => panic!("Unkown entry: {}", field_key),
                }
                let public_key = derive_base64_public_key_from_base64_private_key(&private_key);
                let client_config: ClientConfig = ClientConfig {
                    dns,
                    ip,
                    name,
                    private_key,
                    public_key,
                    output,
                };
                if postpone_config_generation_until_all_defined_ips_are_known {
                    configs_without_ip.push(client_config);
                } else {
                    configs.push(client_config);
                }
            }
            // Loop is done, all specified IPs (servers and clients) are known now - so collect them
            // Then insert IPs into the clients that have not specified them
            let mut used_ips: Vec<Ipv4Addr> = [].to_vec();
            for server in &servers[..] {
                used_ips.push(server.ip)
            }
            for client in &configs[..] {
                used_ips.push(client.ip)
            }
            for client in &configs_without_ip[..] {
                let free_ip = get_next_available_ip(&network.cidrv4, &mut used_ips);
                match free_ip {
                    Some(ip) => {
                        configs.push(ClientConfig {
                            public_key: client.public_key.clone(),
                            ip,
                            output: client.output.clone(),
                            dns: client.dns.clone(),
                            private_key: client.private_key.clone(),
                            name: client.name.clone(),
                        });
                    }
                    None => panic!(
                        "No more IPs available for client {} in network CIDR {}",
                        client.name, &network.cidrv4
                    ),
                }
            }
        } else {
            panic!("Client {} is not a valid TOML table", client_name)
        }
    }

    configs
}

fn get_next_available_ip(network_cidr: &Ipv4Net, used_ips: &mut Vec<Ipv4Addr>) -> Option<Ipv4Addr> {
    let mut available_network_space = network_cidr.hosts();
    'get_ip: loop {
        let ip_to_check = match available_network_space.next() {
            Some(ip) => ip,
            None => panic!(
                "No more IPs available for client in provided CIDR: {}",
                &network_cidr
            ),
        };
        for ip in &used_ips[..] {
            if ip == &ip_to_check {
                continue 'get_ip;
            }
        }

        used_ips.push(ip_to_check);
        return Some(ip_to_check);
    }
}
