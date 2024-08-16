use serde::ser::{Serialize, SerializeStruct};
use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct ClientConfig {
    pub publickey: String,
    pub ip: Ipv4Addr,
    pub output: String,
    pub dns: Option<String>,
    pub privatekey: String,
    pub name: String,
    pub encryption: String,
}
impl Serialize for ClientConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut client;

        if &self.output == "nix" {
            client = serializer.serialize_struct("Client", 5)?;
            client.serialize_field("ip", &self.ip)?;
            client.serialize_field("output", &self.output)?;
            client.serialize_field("encryption", &self.encryption)?;
            client.serialize_field("dns", &self.dns)?;
            client.serialize_field("privatekey", &self.privatekey)?;
            client.end()
        } else {
            client = serializer.serialize_struct("Client", 4)?;
            client.serialize_field("ip", &self.ip)?;
            client.serialize_field("output", &self.output)?;
            client.serialize_field("dns", &self.dns)?;
            client.serialize_field("privatekey", &self.privatekey)?;
            client.end()
        }
    }
}
