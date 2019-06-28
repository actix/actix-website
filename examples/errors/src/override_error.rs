use actix_web::{web, App};
// <override>
use actix_web::{error, http, HttpResponse};
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

fn index() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}
// </override>

fn error2() -> Result<&'static str, MyError> {
    Err(MyError::InternalError)
}

fn error3() -> Result<&'static str, MyError> {
    Err(MyError::Timeout)
}

pub fn main() {
    use actix_web::HttpServer;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/e2", web::get().to(error2))
            .route("/e3", web::get().to(error3))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
