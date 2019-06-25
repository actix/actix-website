// <pbuf>
use actix_web::{web, App, HttpRequest, Result};
use std::path::PathBuf;

fn index(req: HttpRequest) -> Result<String> {
    let path: PathBuf = req.match_info().query("tail").parse().unwrap();
    Ok(format!("Path {:?}", path))
}

pub fn main() {
    use actix_web::HttpServer;

    HttpServer::new(|| App::new().route(r"/a/{tail:.*}", web::get().to(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
// </pbuf>
