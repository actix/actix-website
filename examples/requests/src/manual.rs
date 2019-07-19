// <json-manual>
use actix_web::{error, web, App, Error, HttpResponse};
use bytes::BytesMut;
use futures::{Future, Stream};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

pub fn index_manual(
    payload: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // payload is a stream of Bytes objects
    payload
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        .and_then(|body| {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<MyObj>(&body)?;
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        })
}
// </json-manual>

pub fn main() {
    use actix_web::HttpServer;

    HttpServer::new(|| App::new().route("/", web::post().to_async(index_manual)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
