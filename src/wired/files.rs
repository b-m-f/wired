use serde::Deserialize;
use std::fs::create_dir_all;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct GlobalConfigFromFile {
    pub cidr: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub global: GlobalConfigFromFile,
    pub servers: toml::value::Table,
    pub clients: toml::value::Table,
}

pub fn read_config(config: &String) -> std::io::Result<Config> {
    let content = read_to_string(config)?;
    Ok(toml::from_str(&content)?)
}

pub fn write_config(path: &String, content: &String) {
    let path = Path::new(&path);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("Error writing config file {}: {}", path.display(), e),
    };
    match file.write_all(content.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("Error writing config file {}: {}", path.display(), e),
    }
}

pub fn remove_previous_config_dir(config_dir: &String) {
    if Path::new(config_dir).exists() {
    // delete old configs before regenerating as they have been parsed at this point
    match std::fs::remove_dir_all(&config_dir) {
        Ok(_) => (),
        Err(e) => {
            panic!("Could not remove previous config output directory. Please remove manually and rerun the config generation. Error: {}", e)
        }
    };
    }
}

pub fn create_config_dir(config_dir: &String) {
    match create_dir_all(&config_dir) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };
}
