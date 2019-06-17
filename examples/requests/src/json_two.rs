// // <json-two>
// use actix_web::{error::Error, HttpRequest, HttpResponse};
// use futures::Future;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// struct MyObj {
//     name: String,
//     number: i32,
// }

// fn index(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
//     req.json()
//         .from_err()
//         .and_then(|val: MyObj| {
//             println!("model: {:?}", val);
//             Ok(HttpResponse::Ok().json(val)) // <- send response
//         })
//         .responder()
// }
// // </json-two>
