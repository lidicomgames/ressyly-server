use actix_web::{get, web, HttpResponse, Responder};
use ecies::decrypt;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

use crate::server_vars::AppState;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

// ----------
// Route Server Information and Status
// ----------
#[derive(serde::Serialize)]
struct InfoServer {
    public_key: String,
}

#[get("/info")]
async fn info(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(InfoServer {
        public_key: hex::encode(&data.public_key),
    })
}

// ----------
// ----------

#[derive(serde::Serialize)]
struct UserData {
    name: String,
    private_key: String,
}

#[get("/getPrivateKey/{user}/{password}")]
async fn get_private_key(
    web::Path((user, password)): web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let server_skey = &data.secret_key;

    let hex_password_encrypted = &hex::decode(&password).unwrap();
    let password_decrypted = decrypt(server_skey, hex_password_encrypted).unwrap();

    if password_decrypted.len() < 32 {
        return HttpResponse::BadRequest().body("Password is too short");
    }

    let key = &password_decrypted[0..16];
    let iv = &password_decrypted[16..32];

    let plaintext =
        hex::decode("52831162f9ad3f37edc211db9f474a1a317f3fcc30be3d4fd8840d6437f7e0a5").unwrap();

    let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();

    let cipher_private_key = cipher.encrypt_vec(&plaintext);

    HttpResponse::Ok().json(UserData {
        name: user,
        private_key: hex::encode(cipher_private_key),
    })
}

pub fn get_api_routes() -> actix_web::Scope {
    web::scope("/api").service(info).service(get_private_key)
}
