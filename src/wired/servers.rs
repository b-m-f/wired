use serde::ser::{Serialize, SerializeStruct};
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
    pub encryption: String,
}
impl Serialize for ServerConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut client = serializer.serialize_struct("Server", 8)?;
        client.serialize_field("ip", &self.ip)?;
        client.serialize_field("output", &self.output)?;
        client.serialize_field("encryption", &self.encryption)?;
        client.serialize_field("dns", &self.dns)?;
        client.serialize_field("privatekey", &self.privatekey)?;
        client.serialize_field("listenport", &self.listenport)?;
        client.serialize_field("endpoint", &self.endpoint)?;
        client.serialize_field("persistentkeepalive", &self.persistentkeepalive)?;
        client.end()
    }
}
