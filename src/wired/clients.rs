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
    pub always_rotate_key: bool,
    pub allowedips: Vec<String>,
}
impl Serialize for ClientConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut client;

        let field_count_nix = 6 + if self.allowedips.is_empty() { 0 } else { 1 };
        let field_count_conf = 5 + if self.allowedips.is_empty() { 0 } else { 1 };

        if &self.output == "nix" {
            client = serializer.serialize_struct("Client", field_count_nix)?;
            client.serialize_field("ip", &self.ip)?;
            client.serialize_field("output", &self.output)?;
            client.serialize_field("encryption", &self.encryption)?;
            client.serialize_field("dns", &self.dns)?;
            client.serialize_field("privatekey", &self.privatekey)?;
            client.serialize_field("always-rotate-key", &self.always_rotate_key)?;
            if !self.allowedips.is_empty() {
                client.serialize_field("allowedips", &self.allowedips)?;
            }
            client.end()
        } else {
            client = serializer.serialize_struct("Client", field_count_conf)?;
            client.serialize_field("ip", &self.ip)?;
            client.serialize_field("output", &self.output)?;
            client.serialize_field("dns", &self.dns)?;
            client.serialize_field("privatekey", &self.privatekey)?;
            client.serialize_field("always-rotate-key", &self.always_rotate_key)?;
            if !self.allowedips.is_empty() {
                client.serialize_field("allowedips", &self.allowedips)?;
            }
            client.end()
        }
    }
}
