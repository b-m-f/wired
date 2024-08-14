use std::fs::create_dir_all;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

pub fn read_config(config: &String) -> Result<String, std::io::Error> {
    read_to_string(config)
}

pub fn write_config(path: &String, content: &String) -> Result<(), String> {
    let path = Path::new(&path);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(format!(
                "Error writing config file {}: {}",
                path.display(),
                e
            ))
        }
    };
    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(format!(
                "Error writing config file {}: {}",
                path.display(),
                e
            ))
        }
    }
}

pub fn remove_previous_config_dir(config_dir: &String) -> Result<(), String> {
    if Path::new(config_dir).exists() {
        // delete old configs before regenerating as they have been parsed at this point
        match std::fs::remove_dir_all(&config_dir) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(format!("Could not remove previous config output directory. Please remove manually and rerun the config generation. Error: {e}" ));
            }
        };
    }
    Ok(())
}

pub fn create_config_dir(config_dir: &String) -> Result<(), String> {
    match create_dir_all(&config_dir) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("{}", e)),
    };
}
