// <compress>
use actix_web::{middleware, HttpResponse};

async fn index_br() -> HttpResponse {
    HttpResponse::Ok().body("data")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index_br))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
// </compress>
