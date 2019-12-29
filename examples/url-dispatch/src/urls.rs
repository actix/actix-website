// <url>
use actix_web::{guard, http::header, HttpRequest, HttpResponse, Result};

async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", &["1", "2", "3"])?; // <- generate url for "foo" resource

    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/test/{a}/{b}/{c}")
                    .name("foo") // <- set resource name, then it could be used in `url_for`
                    .guard(guard::Get())
                    .to(|| HttpResponse::Ok()),
            )
            .route("/test/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
// </url>
