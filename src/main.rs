mod cryptography;
mod routes;
mod server_vars;

use cryptography::get_keypair;
use server_vars::AppState;

use actix_cors::Cors;
use actix_web::{main as actix_main, App, HttpServer};
use routes::api::get_api_routes;

#[actix_main]
pub async fn main() -> std::io::Result<()> {
    let (secret_key, public_key) = get_keypair();

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                secret_key,
                public_key,
            })
            .wrap(Cors::default().allow_any_origin().max_age(3600))
            .service(get_api_routes())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
