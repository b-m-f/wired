use super::crypto::get_preshared_key_from_file_or_generate;
use super::files::Config;
use ipnet::IpNet;

#[derive(Debug)]
pub struct NetworkConfig {
    pub cidr: IpNet,
    pub preshared_key: String,
    pub name: String,
    // TODO: extract these bools onto the top-level
    pub rotate_keys: bool,
    pub rotate_ips: bool,
    pub network_type: String,
}

pub fn parse_network_config(
    config: &Config,
    name: &String,
    rotate_keys: bool,
    rotate_ips: bool,
) -> NetworkConfig {
    // TODO: test parsing errors
    let cidr: IpNet = config.network.cidr.parse().unwrap();
    let name: &String = &config.network.name;
    NetworkConfig {
        cidr,
        // TODO: get name from config file
        name: name.to_string(),
        // TODO: make sure this is caught in parsing
        preshared_key: get_preshared_key_from_file_or_generate(&name, rotate_keys),
        rotate_keys,
        rotate_ips,
        // TODO: Parse and set web as default
        // TODO: Make own doc file for network types
        network_type: "web".to_string(),
    }
}
