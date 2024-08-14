use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct ClientConfig {
    pub public_key: String,
    pub ip: Ipv4Addr,
    pub output: String,
    pub dns: Option<String>,
    pub private_key: String,
    pub name: String,
}
