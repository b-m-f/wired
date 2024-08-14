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
