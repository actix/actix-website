#![allow(dead_code)]

// <error-handler>
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, http, web, App, HttpResponse, HttpServer, Result};

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(ErrorHandlers::new().handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                add_error_header,
            ))
            .service(
                web::resource("/")
                    .route(web::get().to(HttpResponse::InternalServerError)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </error-handler>
