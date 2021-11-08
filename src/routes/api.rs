use actix_web::{get, web, Error, HttpResponse, HttpRequest, Responder};
use ecies::decrypt;
use futures::future::{ready, Ready};

use crate::server_vars::AppState;

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
        public_key: hex::encode(&data.public_key.serialize()),
    }
}

// ----------
// ----------

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
                &data.secret_key.serialize(),
                &hex::decode(&password).unwrap()
            )
            .unwrap()
        )
        .unwrap(),
    ))
}

pub fn get_api_routes() -> actix_web::Scope {
    web::scope("/api").service(info).service(get_private_key)
}
