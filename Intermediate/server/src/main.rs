// Only HTTP

use actix_web::{get, App, HttpServer, Responder};
use actix_cors::Cors;

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello Catss!!!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        
        App::new()
            .wrap(cors)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
