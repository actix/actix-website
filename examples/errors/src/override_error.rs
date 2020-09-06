// <override>
use actix_http::ResponseBuilder;
use actix_web::{error, get, http::header, http::StatusCode, App, HttpResponse};
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
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/")]
async fn index() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}
// </override>

#[get("/e2")]
async fn error2() -> Result<&'static str, MyError> {
    Err(MyError::InternalError)
}

#[get("/e3")]
async fn error3() -> Result<&'static str, MyError> {
    Err(MyError::Timeout)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::HttpServer;

    HttpServer::new(|| App::new().service(index).service(error2).service(error3))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
