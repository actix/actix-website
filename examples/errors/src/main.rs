pub mod helpers;
pub mod override_error;
pub mod recommend_one;
pub mod recommend_two;

// <response-error>
use actix_web::{error, HttpRequest, Result};
use failure::Fail;

#[derive(Fail, Debug)]
#[fail(display = "my error")]
pub struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

fn index(_req: HttpRequest) -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}
// </response-error>

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
