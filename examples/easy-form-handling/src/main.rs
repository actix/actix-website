// <easy-form-handling>
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

async fn register(form: web::Form<Register>) -> impl Responder {
    format!("Hello {} from {}!", form.username, form.country)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/register", web::post().to(register))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </easy-form-handling>
