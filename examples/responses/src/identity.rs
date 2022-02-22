// <identity>
use actix_web::{
    get, http::header::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        // v- disable compression
        .insert_header(ContentEncoding::Identity)
        .body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </identity>
