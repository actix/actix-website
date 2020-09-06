use actix_web::{web, App, FromRequest, HttpRequest, HttpServer, Responder};
use serde::Deserialize;

// pub mod custom_handler;
pub mod form;
pub mod json_one;
pub mod json_two;
pub mod multiple;
pub mod path_one;
pub mod path_three;
pub mod path_two;
pub mod query;

#[derive(Deserialize, Debug)]
struct MyInfo {
    username: String,
    id: u32,
}

// <option-one>
async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}
// </option-one>

// <option-two>
async fn extract(req: HttpRequest) -> impl Responder {
    let params = web::Path::<(String, String)>::extract(&req)
        .await
        .unwrap()
        .into_inner();

    let info = web::Json::<MyInfo>::extract(&req)
        .await
        .expect("Err with reading json.");

    format!("{} {} {} {}", params.0, params.1, info.username, info.id)
}
// </option-two>

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{name}/{id}", web::post().to(index))
            .route("/{name}/{id}/extract", web::post().to(extract))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
