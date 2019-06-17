// <json-two>
use actix_web::{error, web, App, FromRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body, max payload size is 4kb
fn index(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

pub fn main() {
    App::new().service(
        web::resource("/")
            .data(
                // change json extractor configuration
                web::Json::<Info>::configure(|cfg| {
                    cfg.limit(4096).error_handler(|err, _req| {
                        // <- create custom error response
                        error::InternalError::from_response(
                            err,
                            HttpResponse::Conflict().finish(),
                        )
                        .into()
                    })
                }),
            )
            .route(web::post().to(index)),
    );
}
// </json-two>
