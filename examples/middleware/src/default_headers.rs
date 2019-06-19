// <default-headers>
use actix_web::{http, middleware, web, App, HttpResponse};

pub fn main() {
    App::new()
        .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
        .service(
            web::resource("/test")
                .route(web::get().to(|| HttpResponse::Ok()))
                .route(
                    web::method(http::Method::HEAD)
                        .to(|| HttpResponse::MethodNotAllowed()),
                ),
        );
}
// </default-headers>
