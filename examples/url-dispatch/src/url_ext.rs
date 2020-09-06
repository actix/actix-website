// <ext>
use actix_web::{HttpRequest, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

    url.into_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .external_resource("youtube", "https://youtube.com/watch/{video_id}")
            .route("/", actix_web::web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
// </ext>
