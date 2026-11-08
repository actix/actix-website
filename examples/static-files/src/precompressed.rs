use std::io;

use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
pub async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        // <battery>
        App::new().service(
            Files::new("/static", "./static")
                .index_file("index.html")
                .try_compressed(),
        )
        // </battery>
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
