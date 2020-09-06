// <error-handler>
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, http, HttpResponse, Result};

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500),
            )
            .service(
                web::resource("/test")
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </error-handler>
