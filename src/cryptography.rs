use ecies::{decrypt, PublicKey, SecretKey};

const PRIVATE_KEY_HEX: &str = "129389004a5ef574f160434d67500513b718b232617f999382c47e799bec4fc1";

pub fn decrypt_to_string(key: &[u8], text: &[u8]) -> String {
    std::str::from_utf8(&decrypt(key, text).unwrap()).unwrap().to_string()
}

pub fn get_keypair() -> (SecretKey, PublicKey) {
    let secret_key = SecretKey::parse_slice(&hex::decode(PRIVATE_KEY_HEX).unwrap()).unwrap();
    let public_key = PublicKey::from_secret_key(&secret_key);

    (secret_key, public_key)
}
