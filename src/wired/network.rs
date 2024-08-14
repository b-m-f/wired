use ipnet::Ipv4Net;
use serde::ser::{Serialize, SerializeStruct};

#[derive(Debug)]
pub struct NetworkConfig {
    pub cidrv4: Ipv4Net,
    pub presharedkey: String,
    pub name: String,
    pub r#type: String,
    // // TODO: extract these bools onto the top-level
    // pub rotate_keys: bool,
    // pub rotate_ips: bool,
}

impl Serialize for NetworkConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut network = serializer.serialize_struct("Network", 4)?;
        network.serialize_field("presharedkey", &self.presharedkey)?;
        network.serialize_field("name", &self.name)?;
        network.serialize_field("type", &self.r#type)?;
        network.serialize_field("cidrv4", &self.cidrv4.to_string())?;
        network.end()
    }
}
