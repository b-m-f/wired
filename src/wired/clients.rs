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
    pub dns: Option<String>,
    pub private_key: String,
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
