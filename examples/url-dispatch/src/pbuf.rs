// <pbuf>
use actix_web::{http::Method, App, HttpRequest, Result};
use std::path::PathBuf;

fn index(req: HttpRequest) -> Result<String> {
    let path: PathBuf = req.match_info().query("tail")?;
    Ok(format!("Path {:?}", path))
}

fn main() {
    App::new()
        .resource(r"/a/{tail:.*}", |r| r.method(Method::GET).f(index))
        .finish();
}
// </pbuf>
