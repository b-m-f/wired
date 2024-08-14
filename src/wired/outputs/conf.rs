use crate::wired::clients::ClientConfig;
use crate::wired::network::NetworkConfig;
use crate::wired::servers::ServerConfig;

pub fn generate_server(
    server: &ServerConfig,
    clients: &Vec<ClientConfig>,
    network: &NetworkConfig,
) -> String {
    let mut server_section = format!(
        "[Interface]\n\
    Address = {}\n\
    ListenPort = {}\n\
    PrivateKey = {}\n",
        server.ip, server.listenport, server.privatekey
    );
    match &server.dns {
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
            peers, peer.name, peer.ip, peer.publickey, network.preshared_key
        );
        peers = format!("{}\n", peers)
    }
    format!("{}\n{}\n", server_section, peers)
}
pub fn generate_client(
    client: &ClientConfig,
    servers: &Vec<ServerConfig>,
    network: &NetworkConfig,
) -> String {
    let mut client_section = format!(
        "[Interface]\n\
    Address = {}\n\
    PrivateKey = {}\n",
        client.ip, client.privatekey
    );
    match &client.dns {
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
            peers, peer.publickey, peer.endpoint, peer.listenport, peer.ip, network.preshared_key
        );
        match &peer.persistentkeepalive {
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
