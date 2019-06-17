// <ext>
use actix_web::{web, App, Error, HttpRequest, HttpResponse, Responder};

fn index(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

    url.into_string()
}

pub fn main() {
    App::new()
        .route("/index.html", web::get().to(index))
        .external_resource("youtube", "https://youtube.com/watch/{video_id}")
        .route("/", actix_web::web::get().to(index));
}
// </ext>
