// <urlencoded>
// use actix_web::{Error, HttpRequest, HttpResponse};
// use futures::future::{ok, Future};
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct FormData {
//     username: String,
// }

// fn index(req: &HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
//     req.urlencoded::<FormData>() // <- get UrlEncoded future
//         .from_err()
//         .and_then(|data| {
//             // <- deserialized instance
//             println!("USERNAME: {:?}", data.username);
//             ok(HttpResponse::Ok().into())
//         })
//         .responder()
// }
// </urlencoded>
fn main() {}
