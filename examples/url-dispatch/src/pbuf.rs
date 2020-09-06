// <pbuf>
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<String> {
    let path: PathBuf = req.match_info().query("tail").parse().unwrap();
    Ok(format!("Path {:?}", path))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route(r"/a/{tail:.*}", web::get().to(index)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
// </pbuf>
