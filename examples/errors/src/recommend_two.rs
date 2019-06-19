use actix_web::App;
// <recommend-two>
use actix_web::{error, fs, http, HttpRequest, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
enum UserError {
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserError::InternalError => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

fn index(_req: HttpRequest) -> Result<&'static str, UserError> {
    fs::NamedFile::open("static/index.html").map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}
// </recommend-two>
pub fn main() {
    App::new().route("/", web::get().to(index));
}
