// <easy-form-handling>
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

fn index() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

fn register(params: web::Form<Register>) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(format!(
        "Hello {} from {}!",
        params.username, params.country
    )))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/register", web::post().to(register))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
// </easy-form-handling>
