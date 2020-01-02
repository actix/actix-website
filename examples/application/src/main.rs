use actix_web::{web, App, HttpResponse};

pub mod app;
pub mod combine;
pub mod config;
pub mod config_app;
pub mod scope;
pub mod state;
pub mod vh;

// <multi>
#[actix_rt::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app1").route("/", web::to(|| HttpResponse::Ok())),
            )
            .service(
                web::scope("/app2").route("/", web::to(|| HttpResponse::Ok())),
            )
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
// </multi>
