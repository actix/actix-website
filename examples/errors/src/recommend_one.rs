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
