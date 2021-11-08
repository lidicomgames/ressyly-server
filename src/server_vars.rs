use ecies::{PublicKey, SecretKey};

pub const PRIVATE_KEY_HEX: &str = "129389004a5ef574f160434d67500513b718b232617f999382c47e799bec4fc1";

pub struct AppState {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}