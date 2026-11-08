use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // <cors>
        App::new().wrap(
            Cors::default()
                .supports_credentials()
                .allowed_origin("https://example.com")
                .allowed_methods(["GET", "POST"])
                .allowed_header("Content-Type"),
        )
        // </cors>
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
