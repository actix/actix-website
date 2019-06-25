// <ext>
use actix_web::{web, App, HttpRequest, Responder};

fn index(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

    url.into_string()
}

pub fn main() {
    use actix_web::HttpServer;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .external_resource("youtube", "https://youtube.com/watch/{video_id}")
            .route("/", actix_web::web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </ext>
