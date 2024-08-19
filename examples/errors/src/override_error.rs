// <override>
use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse,
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display("internal error")]
    InternalError,

    #[display("bad request")]
    BadClientData,

    #[display("timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::HttpServer;

    HttpServer::new(|| App::new().service(index).service(error2).service(error3))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
