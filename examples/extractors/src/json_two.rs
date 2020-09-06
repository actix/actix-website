// <json-two>
use actix_web::{error, web, FromRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body, max payload size is 4kb
async fn index(info: web::Json<Info>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new().service(
            web::resource("/")
                // change json extractor configuration
                .app_data(web::Json::<Info>::configure(|cfg| {
                    cfg.limit(4096).error_handler(|err, _req| {
                        // create custom error response
                        error::InternalError::from_response(
                            err,
                            HttpResponse::Conflict().finish(),
                        )
                        .into()
                    })
                }))
                .route(web::post().to(index)),
        )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </json-two>
