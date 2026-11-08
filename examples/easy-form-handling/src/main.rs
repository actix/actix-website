use actix_web::{App, Either, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;

// <easy-form-handling>
#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}

// Accept JSON or URL-encoded form data.
#[rustfmt::skip]
async fn register(
    form: Either<web::Json<Register>, web::Form<Register>>,
) -> impl Responder {
    let Register { username, country } = form.into_inner();

    format!("Hello {username} from {country}!")
}
// </easy-form-handling>

async fn json_register(form: web::Json<Register>) -> impl Responder {
    format!("Hello {} from {}!", form.username, form.country)
}

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
