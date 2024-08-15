use crate::wired::clients::ClientConfig;
use crate::wired::network::NetworkConfig;
use crate::wired::servers::ServerConfig;

pub fn get_colmena_config(name_of_wg_peer: &String, network_name: &String) -> String {
    format!(
        "
          systemd.tmpfiles.rules = [ \"d /etc/wired 0750 root systemd-network\" ];
          deployment.keys.\"wg-{network_name}.key\" = {{
            keyCommand = [
              \"pass\"
              \"wired/{network_name}/{name_of_wg_peer}.key\"
            ];

            destDir = \"/etc/wired\";
            group = \"systemd-network\";
            permissions = \"0440\";

            uploadAt = \"pre-activation\";
          }};
          deployment.keys.\"wg-{network_name}.psk\" = {{
            keyCommand = [
              \"pass\"
              \"wired/{network_name}/{network_name}.psk\"
            ];

            destDir = \"/etc/wired\";
            group = \"systemd-network\";
            permissions = \"0440\";

            uploadAt = \"pre-activation\";
          }};
            "
    )
}
pub fn get_encryption_privatekey_path(network_name: &String) -> String {
    format!("/etc/wired/wg-{network_name}.key")
}
pub fn get_encryption_psk_path(network_name: &String) -> String {
    format!("/etc/wired/wg-{network_name}.psk")
}

pub fn generate_server(
    server: &ServerConfig,
    clients: &Vec<ClientConfig>,
    network: &NetworkConfig,
) -> String {
    let cidr = network.cidrv4;
    let name = network.name.clone();
    let ip = server.ip;
    let port = server.listenport;
    let encryption = server.encryption.clone();
    let server_name = server.name.clone();

    // create encryption config
    let mut encryption_config = String::new();
    let mut privatekey_path = format!("Use the provided {server_name}.key file");
    let mut psk_path = format!("Use the provided {name}.psk file");
    if encryption == "colmena:pass" {
        privatekey_path = get_encryption_privatekey_path(&name);
        psk_path = get_encryption_psk_path(&name);
        encryption_config = get_colmena_config(&server_name, &name);
    }

    // create peer section
    let mut peers: Vec<String> = Vec::new();

    for client in clients {
        let publickey = client.publickey.clone();
        let ip = client.ip;
        let peer = format!(
            "{{
          wireguardPeerConfig = {{
            PublicKey = \"{publickey}\";
            AllowedIPs =[\"{ip}\"];
            PresharedKeyFile=\"{psk_path}\";
          }};
        }}"
        );
        peers.push(peer);
    }

    let peers: String = peers.into_iter().collect();

    return format!(
        "
{{
  config,
  pkgs,
  lib,
  ...
}}: {{
  {encryption_config}
  networking.firewall.allowedUDPPorts = [20202];
  networking.useNetworkd = true;
  systemd.network.enable = true;
  systemd.network.netdevs.\"50-{name}\" = {{
        netdevConfig = {{
          Kind = \"wireguard\";
          Name = \"{name}\";
          MTUBytes = \"1500\";
        }};
        wireguardConfig = {{
          PrivateKeyFile = \"{privatekey_path}\";
          ListenPort = {port};
        }};
        wireguardPeers = [
          {peers}
        ];
      }};
  systemd.network.networks.{name}= {{
    matchConfig.Name = \"{name}\";
    address = [\"{ip}/32\"];
    routes = [
       {{
          routeConfig = {{
            Destination = \"{cidr}\";
          }};
       }}
    ];
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
    let name = network.name.clone();
    let ip = client.ip;
    let dns = match client.dns.clone() {
        Some(dns) => format!("dns = \"{}\";", dns),
        None => "".to_string(),
    };
    let encryption = client.encryption.clone();

    // create encryption config
    let client_name = client.name.clone();
    let mut encryption_config = String::new();
    let mut privatekey_path = format!("Use the provided {client_name}.key file");
    let mut psk_path = format!("Use the provided {name}.psk file");
    if encryption == "colmena:pass" {
        privatekey_path = get_encryption_privatekey_path(&name);
        psk_path = get_encryption_psk_path(&name);
        encryption_config = get_colmena_config(&client_name, &name);
    }

    // generate peer section
    let mut peers: Vec<String> = Vec::new();

    for server in servers {
        let publickey = server.publickey.clone();
        let ip = server.ip;
        let endpoint = server.endpoint.clone();
        let listenport = server.listenport;
        let persistentkeepalive: String = match server.persistentkeepalive {
            Some(pka) => format!("PersistentKeepalive = {};", pka),
            None => "".to_string(),
        };
        let peer = format!(
            "{{
                   wireguardPeerConfig = {{
                     PublicKey = \"{publickey}\";
                     AllowedIPs = [\"{ip}\"];
                     Endpoint = \"{endpoint}:{listenport}\";
                     {persistentkeepalive}
                     PresharedKeyFile=\"{psk_path}\";
                   }};
                }}"
        );
        peers.push(peer)
    }
    let peers: String = peers.into_iter().collect();

    return format!(
        "{{
          config,
          pkgs,
          lib,
          ...
        }}: {{
          {encryption_config}
          systemd.network.enable = true;
          systemd.network.netdevs.\"10-{name}\"= {{
                netdevConfig = {{
                  Kind = \"wireguard\";
                  Name = \"{name}\";
                  MTUBytes = \"1500\";
                }};
                wireguardConfig = {{
                  #Must be readable by the systemd.network user
                  PrivateKeyFile = \"{privatekey_path}\";
                }};
                wireguardPeers = [
                  {peers}
                ];
              }};
        systemd.network.networks.{name}= {{
              matchConfig.Name = \"{name}\";
              address = [
                \"{ip}/32\"
              ];
              DHCP = \"no\";
              {dns}
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
        }}",
    );
}
