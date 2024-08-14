use ipnet::Ipv4Net;

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
