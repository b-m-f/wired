use super::clients::ClientConfig;
use super::crypto::{derive_base64_public_key_from_base64_private_key, get_private_key};
use super::network::{self, NetworkConfig};
use std::fmt::format;
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct ServerConfig {
    pub endpoint: String,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub persistent_keepalive: Option<u16>,
    pub public_key: String,
    pub dns: Option<String>,
    pub private_key: String,
    pub output: String,
    pub name: String,
}

impl ServerConfig {
    pub fn generate_nix(&self, clients: &Vec<ClientConfig>, network: &NetworkConfig) -> String {
        let cidr = network.cidrv4;
        let name = self.name.clone();
        let ip = self.ip;
        let port = self.port;

        let mut peers: Vec<String> = Vec::new();

        for client in clients {
            let publickey = client.public_key.clone();
            let ip = client.ip;
            let peer = format!(
                "{{
          wireguardPeerConfig = {{
            PublicKey = \"{publickey}\";
            AllowedIPs =[\"{ip}\"];
            PresharedKeyFile=\"UPDATE_THIS_VIA_YOUR_SECRET_MANAGER.\";
          }};
        }}"
            );
            peers.push(peer);
        }

        let peers: String = peers.into_iter().collect();
        // TODO: create peers

        return format!(
            "
{{
  config,
  pkgs,
  lib,
  ...
}}: {{
  networking.firewall.allowedUDPPorts = [20202];
  networking.useNetworkd = true;
  systemd.network = {{
    enable = true;
    netdevs = {{
      \"50-{name}\" = {{
        netdevConfig = {{
          Kind = \"wireguard\";
          Name = \"{name}\";
          MTUBytes = \"1500\";
        }};
        wireguardConfig = {{
          PrivateKeyFile = \"UPDATE_THIS_VIA_YOUR_SECRET_MANAGER.\";
          ListenPort = {port};
        }};
        wireguardPeers = [
          {peers}
        ];
      }};
    }};
    networks.{name}= {{
      matchConfig.Name = \"{name}\";
      address = [\"{ip}\"];
      routes = [
           {{
             routeConfig = {{
               Destination = \"{cidr}\";
             }};
           }}
          ];
         }};
  }};
}}


            "
        );
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
