use core::panic;

use ipnet::Ipv4Net;
use serde::Deserialize;
use std::net::Ipv4Addr;
use std::str::FromStr;

use super::clients::ClientConfig;
use super::crypto::{
    derive_base64_public_key_from_base64_private_key, get_preshared_key, get_private_key,
};
use super::network::NetworkConfig;
use super::servers::ServerConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub network: toml::value::Table,
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
    let network = config.network.clone();

    let mut cidrv4: Ipv4Net = Ipv4Net::new(Ipv4Addr::new(0, 0, 0, 0), 0).unwrap();
    let mut name = "".to_string();
    let mut psk = "".to_string();
    let mut network_type = "".to_string();

    if network.len() == 0 {
        panic!("No network configured")
    }

    for (key, value) in network.iter() {
        match key.as_str() {
            "cidrv4" => match value.get(key) {
                Some(cidr) => {
                    cidrv4 = match Ipv4Net::from_str(&cidr.to_string()) {
                        Ok(cidr) => cidr,
                        Err(e) => panic!("Error when parsing cidrv4 for network: {e}"),
                    }
                }
                None => panic!("Network is missing cidrv4 configuration"),
            },
            "name" => {
                name = match value.get(key) {
                    Some(name) => name.to_string(),
                    None => panic!("No name specified for network"),
                }
            }
            "presharedkey" => {
                psk = match value.get(key) {
                    Some(key) => key.to_string(),
                    None => get_preshared_key(),
                }
            }
            "type" => {
                network_type = match value.get(key) {
                    Some(network_type) => network_type.to_string(),
                    None => "web".to_string(),
                }
            }
            _ => panic!("Unkown field {key} specified for network"),
        }
    }
    // TODO: test parsing errors
    return NetworkConfig {
        cidrv4,
        // TODO: get name from config file
        name,
        // TODO: make sure this is caught in parsing
        presharedkey: psk, // TODO: Parse and set web as default
        // TODO: Make own doc file for network types
        r#type: network_type,
    };
}

pub fn parse_servers(config: &Config) -> Vec<ServerConfig> {
    let servers = &config.servers;
    let network = parse_network(&config);
    // TODO: test parsing error
    let mut configs: Vec<ServerConfig> = Vec::new();
    if servers.len() == 0 {
        panic!("No servers configured")
    }

    for (server_name, server) in servers.iter() {
        if server.is_table() {
            let name = server_name;
            let table = match server.as_table() {
                Some(table) => table,
                None => panic!(
                    "Error when parsing server {}. Client is not a proper TOML table",
                    server_name
                ),
            };
            for (field_key, field_value) in table.iter() {
                let mut privatekey = "".to_string();
                let mut endpoint = "".to_string();
                let mut dns: Option<String> = None;
                let mut ip: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
                let mut pka: Option<u16> = None;
                let mut listenport: u16 = 51820;
                let mut output: String = "".to_string();

                match field_key.as_str() {
                    "privatekey" => {
                        privatekey = match server.get(field_key) {
                            Some(key) => key.to_string().replace("\"", ""),
                            None => get_private_key(),
                        }
                    }
                    "endpoint" => {
                        endpoint = match server.get(field_key) {
                            // TODO: validate that its a correct host
                            Some(endpoint) => endpoint.to_string().replace("\"", ""),
                            None => panic!("Server {} has no endpoint defined", name),
                        }
                    }
                    "dns" => {
                        dns = match server.get(field_key) {
                            Some(dns) => Some(dns.to_string().replace("\"", "")),
                            None => None,
                        }
                    }
                    "ip" => {
                        ip = match server.get(field_key) {
                            // TODO: validate that its a correct host
                            Some(ip) => {
                                match Ipv4Addr::from_str(&ip.to_string().replace("\"", "")) {
                                    Ok(ip) => {
                                        if network.cidrv4.contains(&ip) {
                                            ip
                                        } else {
                                            let cidr = network.cidrv4;
                                            panic!("IP {ip} of server {name} is not in network CIDR {cidr}")
                                        }
                                    }
                                    Err(e) => {
                                        panic!("Error when parsing IP {ip} of server {name}: {e}")
                                    }
                                }
                            }
                            None => panic!("Server {} has no ip defined", name),
                        }
                    }
                    "persistentkeepalive" => pka = match server.get(field_key) {
                        Some(pka) => match pka.as_integer() {
                            Some(pka) => match u16::try_from(pka){
                                Ok(pka) => Some(pka),
                                    Err(e) => panic!("Error when parsing persistentkeepalive '{pka}' for server {name}: {e}")
                            }
                                ,
                            None => panic!(
                                "Incorrect persistentkeepalive {field_value} configured for server {name}"
                            ),
                        },
                        None => None,
                    },
                    "listenport" => listenport = match server.get(field_key){
                        Some(port) => match port.as_integer(){
                            Some(port) =>
                                match u16::try_from(port){
                                    Ok(port) => port,
                                    Err(e) => panic!("Error when parsing listenport '{port}' for server {name}: {e}")
                                }
                            ,None=> panic!("Incorrect listenport '{port}' specified for server {name}")

                        },
                        None => panic!("Missing listenport for server {name}"),

                    },
                    "output" => output = match server.get(field_key){
                        Some(output) => {
                        let output_checked = match output.to_string().as_str(){
                            "conf" => output.to_string(),
                            "nix" =>output.to_string(),
                            _ => panic!("Unkown output {output} for server {name}")
                        };
                        output_checked
                        },
                        None => "conf".to_string(),
                    },
                    _ => panic!("Unkown entry '{}' for server {name}", field_key),
                }
                let publickey = derive_base64_public_key_from_base64_private_key(&privatekey);
                let server_config = ServerConfig {
                    dns,
                    endpoint,
                    privatekey,
                    publickey,
                    listenport,
                    output,
                    name: name.to_string(),
                    ip,
                    persistentkeepalive: pka,
                };
                configs.push(server_config);
            }
        } else {
            panic!("Server {} is not a valid TOML table", server_name)
        }
    }
    configs
}

pub fn parse_clients(config: &Config) -> Vec<ClientConfig> {
    let clients = &config.clients;
    let servers = parse_servers(&config);
    let network = parse_network(&config);

    if clients.len() == 0 && network.r#type == "web" {
        panic!("No clients configured")
    }

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
                let mut privatekey: String = "".to_string();
                let mut ip: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
                let mut dns: Option<String> = None;
                let mut output: String = "conf".to_string();
                let mut postpone_config_generation_until_all_defined_ips_are_known = false;
                println!("{}={}", field_key, field_value);
                match field_key.as_str() {
                    "privatekey" => {
                        privatekey = match client.get(field_key) {
                            Some(key) => key.to_string().replace("\"", ""),
                            None => get_private_key(),
                        }
                    }
                    "ip" => {
                        ip = match client.get(field_key) {
                            Some(ip) => {
                                match Ipv4Addr::from_str(&ip.to_string().replace("\"", "")) {
                                    Ok(ip) => {
                                        if network.cidrv4.contains(&ip) {
                                            ip
                                        } else {
                                            let cidr = network.cidrv4;
                                            panic!("IP {ip} of client {name} is not in network CIDR {cidr}")
                                        }
                                    }
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
                    _ => panic!("Unkown entry '{}' for client {name}", field_key),
                }
                let publickey = derive_base64_public_key_from_base64_private_key(&privatekey);
                let client_config: ClientConfig = ClientConfig {
                    dns,
                    ip,
                    name,
                    privatekey,
                    publickey,
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
                            publickey: client.publickey.clone(),
                            ip,
                            output: client.output.clone(),
                            dns: client.dns.clone(),
                            privatekey: client.privatekey.clone(),
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
