use crate::wired::clients::ClientConfig;
use crate::wired::network::NetworkConfig;
use crate::wired::servers::ServerConfig;

pub fn generate_server(
    server: &ServerConfig,
    clients: &Vec<ClientConfig>,
    network: &NetworkConfig,
) -> String {
    let cidr = network.cidrv4;
    let name = server.name.clone();
    let ip = server.ip;
    let port = server.port;

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
pub fn generate_client(
    client: &ClientConfig,
    servers: &Vec<ServerConfig>,
    network: &NetworkConfig,
) -> String {
    // TODO: Create peer config
    // TODO: make shareable with ServerConfig
    let name = network.name.clone();
    let ip = client.ip;
    let dns = match client.dns.clone() {
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
