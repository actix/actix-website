// <recommend-one>
use actix_web::{error, http, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
enum UserError {
    #[fail(display = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserError::ValidationError { .. } => {
                HttpResponse::new(http::StatusCode::BAD_REQUEST)
            }
        }
    }
}
// </recommend-one>
fn index() -> Result<&'static str, UserError> {
    Err(UserError::ValidationError {
        field: "bad stuff".to_string(),
    })
}

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
