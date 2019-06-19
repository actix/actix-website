use actix_web::{web, App};
// <helpers>
use actix_web::{error, HttpRequest, Result};

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

pub fn index(_req: HttpRequest) -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "test" });

    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}
// </helpers>
pub fn main() {
    App::new().route("/", web::get().to(index));
}
