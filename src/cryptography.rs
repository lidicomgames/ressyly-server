use ecies::{PublicKey, SecretKey};

const PRIVATE_KEY_HEX: &str = "129389004a5ef574f160434d67500513b718b232617f999382c47e799bec4fc1";

pub fn get_keypair() -> ([u8; 32], [u8; 33]) {
    let secret_key = SecretKey::parse_slice(&hex::decode(PRIVATE_KEY_HEX).unwrap()).unwrap();
    let public_key = PublicKey::from_secret_key(&secret_key);

    (secret_key.serialize(), public_key.serialize_compressed())
}
