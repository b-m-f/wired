use super::crypto::get_preshared_key_from_file_or_generate;
use super::files::Config;
use ipnet::IpNet;

#[derive(Debug)]
pub struct GlobalConfig {
    pub cidr: IpNet,
    pub preshared_key: String,
    pub name: String,
    pub rotate_keys: bool,
    pub rotate_ips: bool,
}

pub fn parse_global_config(
    config: &Config,
    name: &String,
    rotate_keys: bool,
    rotate_ips: bool,
) -> GlobalConfig {
    // TODO: test parsing error
    let cidr: IpNet = config.global.cidr.parse().unwrap();
    GlobalConfig {
        cidr,
        name: name.to_string(),
        preshared_key: get_preshared_key_from_file_or_generate(name, rotate_keys),
        rotate_keys,
        rotate_ips,
    }
}
