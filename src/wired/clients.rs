use std::net::Ipv4Addr;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ClientConfig {
    pub publickey: String,
    pub ip: Ipv4Addr,
    pub output: String,
    pub dns: Option<String>,
    pub privatekey: String,
    pub name: String,
}
