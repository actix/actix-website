// <override>
use actix_web::{error, http, HttpRequest, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
enum MyError {
    #[fail(display = "internal error")]
    InternalError,
    #[fail(display = "bad request")]
    BadClientData,
    #[fail(display = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::InternalError => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            MyError::BadClientData => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            MyError::Timeout => HttpResponse::new(http::StatusCode::GATEWAY_TIMEOUT),
        }
    }
}

fn index(req: &HttpRequest) -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}
// </override>
