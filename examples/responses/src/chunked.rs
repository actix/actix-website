// <chunked>
// use actix_web::{web, HttpRequest, HttpResponse};
// use bytes::Bytes;
// use futures::stream::once;

// fn index(req: HttpRequest) -> HttpResponse {
//     HttpResponse::Ok()
//         .chunked()
//         .body(Body::Streaming(Box::new(once(Ok(Bytes::from_static(
//             b"data",
//         ))))))
// }
// </chunked>
pub fn main() {}
