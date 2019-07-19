// <recommend-two>
use actix_web::{error, http, HttpResponse};
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

fn index() -> Result<&'static str, UserError> {
    do_thing_that_failes().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}
// </recommend-two>

fn do_thing_that_failes() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "some error"))
}

pub fn main() {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
