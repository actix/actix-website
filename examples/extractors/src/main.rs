use actix_web::{web, App, FromRequest, HttpRequest, HttpServer, Responder};
use futures::future::Future;
use serde::Deserialize;

// mod custom_handler;
mod form;
mod json_one;
mod json_two;
mod multiple;
mod path_one;
mod path_two;
mod query;

#[derive(Deserialize, Debug)]
struct MyInfo {
    username: String,
    id: u32,
}

// <main>
// Option 1:  passed as a parameter to a handler function
fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}

// Option 2:  accessed by calling extract() on the Extractor
fn extract(req: HttpRequest) -> impl Responder {
    let params = web::Path::<(String, String)>::extract(&req).unwrap();

    let info = web::Json::<MyInfo>::extract(&req)
        .wait()
        .expect("Err with reading json.");

    format!("{} {} {} {}", params.0, params.1, info.username, info.id)
}
// </main>

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/{name}/{id}", web::post().to(index))
            .route("/{name}/{id}/extract", web::post().to(extract))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
