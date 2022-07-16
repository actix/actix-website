use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
// <easy-form-handling>
use actix_web::web::{Either, Json, Form};

#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

// register form is JSON
async fn json_register(form: web::Json<Register>) -> impl Responder {
    format!("Hello {} from {}!", form.username, form.country)
}

// register form can be either JSON or URL-encoded
async fn register(form: Either<Json<Register>, Form<Register>>) -> impl Responder {
    let Register { username, country } = form.into_inner();
    format!("Hello {username} from {country}!")
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
            .route("/json_register", web::post().to(json_register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
