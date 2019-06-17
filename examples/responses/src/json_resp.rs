// <json-resp>
use actix_web::{web, App, HttpRequest, Result};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
}

fn index(req: HttpRequest) -> Result<web::Json<MyObj>> {
    Ok(web::Json(MyObj {
        name: req.match_info().query("name").to_string(),
    }))
}

fn main() {
    App::new().route(r"/a/{name}", web::get().to(index));
}
// </json-resp>
