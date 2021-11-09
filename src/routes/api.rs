use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

use crate::server_vars::AppState;
use crate::cryptography::decrypt_to_string;

// ----------
// Route Server Information and Status
// ----------
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
        public_key: hex::encode(&data.public_key.serialize_compressed()),
    }
}

// ----------
// ----------

#[get("/getPrivateKey/{user}/{password}")]
async fn get_private_key(
    web::Path((user, password)): web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let key_serialize = &data.secret_key.serialize();
    let text_decrypted = &hex::decode(&password).unwrap();

    HttpResponse::Ok().body(format!(
        "user: {}\npassword {}\nmessage {}",
        &user.clone(),
        &password,
        decrypt_to_string(key_serialize, text_decrypted)
    ))
}

pub fn get_api_routes() -> actix_web::Scope {
    web::scope("/api").service(info).service(get_private_key)
}
