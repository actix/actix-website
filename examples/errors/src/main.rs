pub mod helpers;
pub mod override_error;
pub mod recommend_one;
use actix_web::{web, App};
// <response-error>
use actix_web::{error, HttpRequest};
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
    App::new().route("/", web::get().to(index));
}
