use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

// <easy-form-handling>
#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

async fn register(form: web::Form<Register>) -> impl Responder {
    format!("Hello {} from {}!", form.username, form.country)
}
// </easy-form-handling>

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

#[actix_web::main]
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
