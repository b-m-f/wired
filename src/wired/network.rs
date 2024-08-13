use super::crypto::get_preshared_key;
use super::files::Config;
use ipnet::Ipv4Net;

#[derive(Debug)]
pub struct NetworkConfig {
    pub cidrv4: Ipv4Net,
    pub preshared_key: String,
    pub name: String,
    pub r#type: String,
    // TODO: extract these bools onto the top-level
    pub rotate_keys: bool,
    pub rotate_ips: bool,
}

pub fn parse_network_config(config: &Config, rotate_keys: bool, rotate_ips: bool) -> NetworkConfig {
    // TODO: test parsing errors
    let cidrv4: Ipv4Net = config.network.cidrv4.parse().unwrap();
    let name: &String = &config.network.name;
    NetworkConfig {
        cidrv4,
        // TODO: get name from config file
        name: name.to_string(),
        // TODO: make sure this is caught in parsing
        preshared_key: match &config.network.presharedkey {
            Some(psk) => psk.to_string(),
            None => get_preshared_key(),
        },
        rotate_keys,
        rotate_ips,
        // TODO: Parse and set web as default
        // TODO: Make own doc file for network types
        r#type: "web".to_string(),
    }
}
