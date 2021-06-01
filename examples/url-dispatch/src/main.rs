pub mod cfg;
pub mod dhandler;
pub mod guard;
pub mod guard2;
pub mod minfo;
pub mod norm;
pub mod norm2;
pub mod path;
pub mod path2;
pub mod resource;
pub mod scope;
pub mod url_ext;
pub mod urls;

// <main>
use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/user", web::post().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </main>
