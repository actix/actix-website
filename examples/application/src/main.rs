use actix_web::{web, App, HttpResponse, HttpServer};

pub mod app;
pub mod combine;
pub mod config;
pub mod scope;
pub mod state;
pub mod vh;

// <multi>
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/app1").route("/", web::to(HttpResponse::Ok)))
            .service(web::scope("/app2").route("/", web::to(HttpResponse::Ok)))
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </multi>
