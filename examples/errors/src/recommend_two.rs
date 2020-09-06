// <recommend-two>
use actix_http::ResponseBuilder;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use failure::Fail;

#[derive(Fail, Debug)]
enum UserError {
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

async fn index() -> Result<&'static str, UserError> {
    do_thing_that_failes().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}
// </recommend-two>

fn do_thing_that_failes() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "some error"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
