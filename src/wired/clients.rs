use ipnet::Ipv4Net;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::str::FromStr;

use super::crypto::{derive_base64_public_key_from_base64_private_key, get_private_key};
use super::network::{self, NetworkConfig};
use super::servers::ServerConfig;

#[derive(Debug)]
pub struct ClientConfig {
    pub public_key: String,
    pub ip: Ipv4Addr,
    pub output: String,
    dns: Option<String>,
    private_key: String,
    pub name: String,
}

impl ClientConfig {
    pub fn generate_nix(&self, servers: &Vec<ServerConfig>, network: &NetworkConfig) -> String {
        // TODO: Create peer config
        // TODO: make shareable with ServerConfig
        let name = network.name.clone();
        let ip = self.ip;
        let dns = match self.dns.clone() {
            Some(dns) => format!("dns = \"{}\"", dns),
            None => "".to_string(),
        };
        let mut peers: Vec<String> = Vec::new();

        for server in servers {
            let publickey = server.public_key.clone();
            let ip = server.ip;
            let endpoint = server.endpoint.clone();
            let listenport = server.port;
            let persistentkeepalive: String = match server.persistent_keepalive {
                Some(pka) => format!("PersistentKeepalive = {};", pka),
                None => "".to_string(),
            };
            let peer = format!(
                "{{
                   wireguardPeerConfig = {{
                     PublicKey = \"{publickey}\";
                     AllowedIPs = [\"{ip}\"];
                     Endpoint = \"{endpoint}:{listenport}\"
                     {persistentkeepalive}
                     PresharedKeyFile=\"UPDATE_THIS_VIA_YOUR_SECRET_MANAGER.\"
                   }};
                }}"
            );
            peers.push(peer)
        }
        let peers: String = peers.into_iter().collect();

        // TODO: add integration of secret manager
        return format!(
            "{{
          config,
          pkgs,
          lib,
          ...
        }}: {{
          systemd.network = {{
            enable = true;
            netdevs = {{
              \"10-{name}\" = {{
                netdevConfig = {{
                  Kind = \"wireguard\";
                  Name = \"{name}\";
                  MTUBytes = \"1500\";
                }};
                wireguardConfig = {{
                  #Must be readable by the systemd.network user
                  PrivateKeyFile = \"UPDATE_THIS_VIA_YOUR_SECRET_MANAGER.\"
                }};
                wireguardPeers = [
                  {peers}
                ];
              }};
            }};
            networks.{name}= {{
              matchConfig.Name = \"{name}\";
              address = [
                \"{ip}/32\"
              ];
              DHCP = \"no\";
              dns = \"{dns}\";
              networkConfig = {{
                IPv6AcceptRA = false;
              }};
              routes = [
                   {{
                     routeConfig = {{
                       Destination = 10.10.10.0/24;
                     }};
                   }}
                  ];
            }};
          }};
        }}",
        );
    }
    pub fn generate_conf(&self, servers: &Vec<ServerConfig>, pre_shared_key: &String) -> String {
        let mut client_section = format!(
            "[Interface]\n\
    Address = {}\n\
    PrivateKey = {}\n",
            self.ip, self.private_key
        );
        match &self.dns {
            Some(dns) => {
                client_section = format!(
                    "{}\
             DNS = {}",
                    client_section, dns,
                )
            }
            None => (),
        }
        let mut peers = "".to_string();
        for peer in &servers[..] {
            peers = format!(
                "{}\n\
        [Peer]\n\
        PublicKey = {}\n\
        Endpoint = {}:{}\n\
        AllowedIPs = {}\n\
        PresharedKey = {}",
                peers, peer.public_key, peer.endpoint, peer.port, peer.ip, pre_shared_key
            );
            match &peer.persistent_keepalive {
                Some(ka) => {
                    peers = format!(
                        "{}\n\
             PersistentKeepalive = {}",
                        peers, ka,
                    )
                }
                None => (),
            }
            peers = format!("{}\n", peers)
        }
        format!("{}\n{}\n", client_section, peers)
    }
}

pub fn parse_client_configs(
    clients: toml::value::Table,
    server_configs: &Vec<ServerConfig>,
    network: &NetworkConfig,
) -> Vec<ClientConfig> {
    // TODO: test parsing error
    let mut configs: Vec<ClientConfig> = Vec::new();
    let mut used_ips: Vec<Ipv4Addr> = [].to_vec();
    for server in server_configs {
        used_ips.push(server.ip)
    }

    let mut clients_without_ip = HashMap::new();

    for (key, value) in clients.iter() {
        let private_key = match value.get("privatekey") {
            Some(key) => key.to_string().replace("\"", ""),
            None => get_private_key(),
        };
        let ip: Option<String> = match value.get("ip") {
            Some(ip) => Some(ip.to_string().replace("\"", "")),
            None => {
                clients_without_ip.insert(key, value);
                None
            }
        };
        match ip {
            Some(ip) => {
                let client_config: ClientConfig = ClientConfig {
                    ip: match Ipv4Addr::from_str(&ip) {
                        Ok(ip) => ip,
                        // TODO: catch error
                        Err(e) => panic!("{}", e),
                    },
                    name: key.to_string(),
                    dns: match value.get("dns") {
                        Some(dns) => Some(dns.to_string().replace("\"", "")),
                        None => None,
                    },
                    public_key: derive_base64_public_key_from_base64_private_key(&private_key),
                    private_key,
                    output: match value.get("output") {
                        Some(r#type) => r#type.to_string().replace("\"", ""),
                        None => "conf".to_string(),
                    },
                };

                configs.push(client_config);
            }

            None => {
                // Refer to explanation on the second loop
                clients_without_ip.insert(key, value);
                ()
            }
        }
    }
    // This has to be done to make sure that newly added clients don't assign early IP addresses,
    // that might be already in use, but whose config files were not parsed yet.
    // Running a second loop over the clients that have no IP set -> probably because they have no
    // config file makes sure, that the used_ips accumulator is filled with existing configs before
    // new IPs are given out
    for (key, value) in clients_without_ip.iter() {
        let private_key = match value.get("privatekey") {
            Some(key) => key.to_string().replace("\"", ""),
            None => get_private_key(),
        };
        let free_ip = get_next_available_ip(&network.cidrv4, &mut used_ips);
        match free_ip {
            Some(ip) => {
                let client_config: ClientConfig = ClientConfig {
                    ip,
                    name: key.to_string(),
                    dns: match value.get("dns") {
                        Some(dns) => Some(dns.to_string().replace("\"", "")),
                        None => None,
                    },
                    public_key: derive_base64_public_key_from_base64_private_key(&private_key),
                    private_key,
                    output: match value.get("output") {
                        Some(r#type) => r#type.to_string().replace("\"", ""),
                        None => "conf".to_string(),
                    },
                };

                configs.push(client_config);
            }

            None => panic!(
                "No more IPs available for client in provided CIDR: {}",
                &network.cidrv4
            ),
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
