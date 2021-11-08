mod routes;
mod server_vars;

use server_vars::{AppState, PRIVATE_KEY_HEX};

use actix_cors::Cors;
use actix_web::{main as actix_main, App, HttpServer};
use routes::api::get_api_routes;

use ecies::{PublicKey, SecretKey};

#[actix_main]
pub async fn main() -> std::io::Result<()> {
    let secret_key = SecretKey::parse_slice(&hex::decode(PRIVATE_KEY_HEX).unwrap()).unwrap();
    let public_key = PublicKey::from_secret_key(&secret_key);

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                secret_key: secret_key.clone(),
                public_key: public_key.clone(),
            })
            .wrap(Cors::default().allow_any_origin().max_age(3600))
            .service(get_api_routes())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
