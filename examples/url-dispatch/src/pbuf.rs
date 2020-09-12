// <pbuf>
use actix_web::{get, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

#[get("/a/{tail:.*}")]
async fn index(req: HttpRequest) -> Result<String> {
    let path: PathBuf = req.match_info().query("tail").parse().unwrap();
    Ok(format!("Path {:?}", path))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
// </pbuf>
