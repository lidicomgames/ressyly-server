use actix_cors::Cors;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use ecies::{decrypt, encrypt, utils::generate_keypair, PublicKey, SecretKey};
use futures::future::{ready, Ready};

const PRIVATE_KEY_HEX: &str = "129389004a5ef574f160434d67500513b718b232617f999382c47e799bec4fc1";

struct AppState {
    keypair: (SecretKey, PublicKey),
}

#[derive(serde::Serialize)]
struct InfoServer {
    public_key: String,
}

impl Responder for InfoServer {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok().json(self)))
    }
}

#[get("/info")]
async fn info(data: web::Data<AppState>) -> impl Responder {
    InfoServer {
        public_key: hex::encode(&data.keypair.1.serialize()),
    }
}

#[get("/getPrivateKey/{user}/{password}")]
async fn get_private_key(
    web::Path((user, password)): web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "user: {}\npassword {}\nmessage {}",
        &user.clone(),
        &password,
        std::str::from_utf8(
            &decrypt(
                &data.keypair.0.serialize(),
                &hex::decode(&password).unwrap()
            )
            .unwrap()
        ).unwrap(),
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = SecretKey::parse_slice(&hex::decode(PRIVATE_KEY_HEX).unwrap()).unwrap();
    let public_key = PublicKey::from_secret_key(&secret_key);

    let app_state = web::Data::new(AppState {
        keypair: (secret_key.clone(), public_key.clone()),
    });

    HttpServer::new(move || {
        let api = web::scope("/api").service(info).service(get_private_key);
        App::new()
            .app_data(app_state.clone())
            .wrap(Cors::default().allow_any_origin().max_age(3600))
            .service(api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
