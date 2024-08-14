use super::command;

pub fn derive_base64_public_key_from_base64_private_key(key: &String) -> Result<String, String> {
    return command::run_with_input_on_stdin(
        "wg".to_string(),
        &["pubkey".to_string()],
        key.to_string(),
    );
}

pub fn get_private_key() -> Result<String, String> {
    return command::run("wg".to_string(), &["genkey".to_string()]);
}

pub fn get_preshared_key() -> Result<String, String> {
    return command::run("wg".to_string(), &["genpsk".to_string()]);
}
