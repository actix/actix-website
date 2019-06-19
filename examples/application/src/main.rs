use actix_web::{web, App, HttpResponse};

pub mod app;
pub mod combine;
pub mod config;
pub mod scope;
pub mod state;
pub mod vh;

#[rustfmt::skip]
// <multi>
fn main() {
    App::new()
        .service(
            web::scope("/app1")
                .route("/", web::to(|| HttpResponse::Ok())))
        .service(
            web::scope("/app2")
                .route("/", web::to(|| HttpResponse::Ok())))
        .route("/", web::to(|| HttpResponse::Ok()));
}
// </multi>
