// <recommend-one>
use actix_http::ResponseBuilder;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
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
async fn index() -> Result<&'static str, UserError> {
    Err(UserError::ValidationError {
        field: "bad stuff".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
