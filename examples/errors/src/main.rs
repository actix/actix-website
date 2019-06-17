mod helpers;
mod override_error;
mod recommend_one;
// <response-error>
use actix_web::{error, HttpRequest};
use failure::Fail;

#[derive(Fail, Debug)]
#[fail(display = "my error")]
struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

fn index(req: HttpRequest) -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}
// </response-error>
fn main() {}
