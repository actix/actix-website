// <form-optional>
use actix_web::{post, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// accept form data when it is present and valid
#[post("/maybe")]
async fn maybe(form: Option<web::Form<FormData>>) -> Result<String> {
    let Some(form) = form else {
        return Ok("Missing or invalid form data.".to_string());
    };

    Ok(format!("Welcome {}!", form.username))
}
// </form-optional>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(maybe))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
