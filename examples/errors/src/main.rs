pub mod helpers;
pub mod logging;
pub mod override_error;
pub mod recommend_one;
pub mod recommend_two;

// <response-error>
use actix_web::{error, Result};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
#[display("my error: {name}")]
struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

async fn index() -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}
// </response-error>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
