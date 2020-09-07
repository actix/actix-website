// <recommend-one>
use actix_http::ResponseBuilder;
use actix_web::{error, get, http::header, http::StatusCode, App, HttpResponse, HttpServer};
use failure::Fail;

#[derive(Fail, Debug)]
enum UserError {
    #[fail(display = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
// </recommend-one>

#[get("/")]
async fn index() -> Result<&'static str, UserError> {
    Err(UserError::ValidationError {
        field: "bad stuff".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
