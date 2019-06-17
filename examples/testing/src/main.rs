mod integration_one;
mod integration_three;
mod integration_two;
mod stream_response;
mod websockets;
// <unit-tests>
use actix_web::{http, test, HttpRequest, HttpResponse};

fn index(req: HttpRequest) -> HttpResponse {
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            return HttpResponse::Ok().into();
        }
    }
    HttpResponse::BadRequest().into()
}

fn main() {
    let req =
        test::TestRequest::with_header("content-type", "text/plain").to_http_request();

    let resp = test::block_on(index(req)).unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);

    let req = test::TestRequest::default().to_http_request();
    let resp = test::block_on(index(req)).unwrap();
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}
// </unit-tests>
