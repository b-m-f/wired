use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct ServerConfig {
    pub endpoint: String,
    pub ip: Ipv4Addr,
    pub listenport: u16,
    pub persistentkeepalive: Option<u16>,
    pub publickey: String,
    pub dns: Option<String>,
    pub privatekey: String,
    pub output: String,
    pub name: String,
}
