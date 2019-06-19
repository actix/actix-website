use actix_web::{web, App, HttpRequest};
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
pub fn main() {
    App::new().route("/", web::get().to(index));
}

fn index(_req: HttpRequest) -> Result<&'static str, UserError> {
    Err(UserError::ValidationError {
        field: "bad stuff".to_string(),
    })
}
