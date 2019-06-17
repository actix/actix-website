// <helpers>
use actix_web::{error, HttpRequest, Result};

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

fn index(req: &HttpRequest) -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "test" });

    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}
// </helpers>
