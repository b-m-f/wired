use log::debug;
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::{read_dir, File};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use x25519_dalek::{PublicKey, StaticSecret};

struct KeyPairBase64 {
    public: String,
    private: String,
}

fn generate_base64_keypair() -> KeyPairBase64 {
    let secret = StaticSecret::new(OsRng);
    let public = PublicKey::from(&secret);
    KeyPairBase64 {
        private: base64::encode(secret.to_bytes()),
        public: base64::encode(public.to_bytes()),
    }
}

pub fn derive_base64_public_key_from_base64_private_key(key: &String) -> String {
    let bytes_for_key = match base64::decode(key) {
        Ok(bytes) => bytes,
        Err(e) => panic!("{}", e),
    };
    let mut raw_private_key_buffer = [0; 32];
    raw_private_key_buffer.copy_from_slice(&bytes_for_key[..32]);

    let priv_key: StaticSecret = StaticSecret::from(raw_private_key_buffer);
    let public_key = PublicKey::from(&priv_key);

    base64::encode(public_key.to_bytes())
}

pub fn get_private_key_from_file_or_generate(path: &Path, rotate_keys: bool) -> String {
    if !rotate_keys {
        let mut key: String = String::from("");
        match File::open(&path) {
            Ok(file) => {
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            if line.contains("PrivateKey =") {
                                let key_from_file = line.replace("PrivateKey =", "");

                                key = key_from_file.trim().to_string();
                            }
                        }
                        Err(e) => {
                            debug!(
                                "Error when parsing {} for private key: {}",
                                path.to_str().unwrap(),
                                e
                            )
                        }
                    }
                }
            }
            Err(_) => {
                let keypair = generate_base64_keypair();
                key = keypair.private;
            }
        };
        return key;
    } else {
        let keypair = generate_base64_keypair();
        return keypair.private;
    }
}

pub fn get_preshared_key_from_file_or_generate(network_name: &String, rotate_keys: bool) -> String {
    let paths = match read_dir(format!("./{}", network_name)) {
        Ok(paths) => paths,
        Err(_) => return generate_pre_shared_key(),
    };

    if !rotate_keys {
        let mut key: String = String::from("");
        for dir_entry in paths {
            match dir_entry {
                Ok(dir_entry) => {
                    let path_as_string: String = match dir_entry.path().to_str() {
                        Some(path) => path.to_string(),
                        None => "".to_string(),
                    };

                    match File::open(&dir_entry.path()) {
                        Ok(file) => {
                            let reader = BufReader::new(file);

                            for line in reader.lines() {
                                match line {
                                    Ok(line) => {
                                        if line.contains("PresharedKey =") {
                                            let key_from_file = line.replace("PresharedKey =", "");

                                            key = key_from_file.trim().to_string();
                                        }
                                    }
                                    Err(e) => {
                                        debug!(
                                            "Error when reading {} for preshared key: {}",
                                            path_as_string, e
                                        )
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            key = generate_pre_shared_key();
                        }
                    };
                }
                Err(e) => panic!(
                    "Error when parsing existing files for PreShared Keys: {}",
                    e
                ),
            }
        }
        return key;
    } else {
        return generate_pre_shared_key();
    }
}

fn generate_pre_shared_key() -> String {
    let mut csprng = OsRng {};
    let mut key = [0u8; 32];
    csprng.fill_bytes(&mut key);
    return base64::encode(key);
}
