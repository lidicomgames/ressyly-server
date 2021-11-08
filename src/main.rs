use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/info")]
async fn index() -> impl Responder {
    "Hola mundo"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let api = web::scope("/api").service(index);
        App::new().service(api)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
