// <ext>
use actix_web::{App, Error, HttpRequest, HttpResponse};

fn index(req: &HttpRequest) -> Result<HttpResponse, Error> {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"])?;
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");
    Ok(HttpResponse::Ok().into())
}

fn main() {
    let app = App::new()
        .resource("/index.html", |r| r.f(index))
        .external_resource("youtube", "https://youtube.com/watch/{video_id}")
        .finish();
}
// </ext>
