// <url>
use actix_web::{http::header, http::Method, App, HttpRequest, HttpResponse, Result};

fn index(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", &["1", "2", "3"])?; // <- generate url for "foo" resource
    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

fn main() {
    let app = App::new()
        .resource("/test/{a}/{b}/{c}", |r| {
            r.name("foo"); // <- set resource name, then it could be used in `url_for`
            r.method(Method::GET).f(|_| HttpResponse::Ok());
        })
        .route("/test/", Method::GET, index)
        .finish();
}
// </url>
