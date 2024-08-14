use rand::rngs::OsRng;
use rand::RngCore;
use x25519_dalek::{PublicKey, StaticSecret};

struct KeyPairBase64 {
    private: String,
}

fn generate_base64_keypair() -> KeyPairBase64 {
    let secret = StaticSecret::new(OsRng);
    KeyPairBase64 {
        private: base64::encode(secret.to_bytes()),
    }
}

pub fn derive_base64_public_key_from_base64_private_key(key: &String) -> Result<String, String> {
    let bytes_for_key = match base64::decode(key) {
        Ok(bytes) => bytes,
        Err(e) => return Err(format!("{}", e)),
    };
    let mut raw_private_key_buffer = [0; 32];
    raw_private_key_buffer.copy_from_slice(&bytes_for_key[..32]);

    let priv_key: StaticSecret = StaticSecret::from(raw_private_key_buffer);
    let public_key = PublicKey::from(&priv_key);

    Ok(base64::encode(public_key.to_bytes()))
}

pub fn get_private_key() -> String {
    let keypair = generate_base64_keypair();
    return keypair.private;
}

pub fn get_preshared_key() -> String {
    let mut csprng = OsRng {};
    let mut key = [0u8; 32];
    csprng.fill_bytes(&mut key);
    return base64::encode(key);
}
