use ecies::{PublicKey, SecretKey};

pub struct AppState {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}