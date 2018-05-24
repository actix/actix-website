// <minfo>
use actix_web::{App, HttpRequest, Result};

fn index(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().query("v1")?;
    let v2: u8 = req.match_info().query("v2")?;
    Ok(format!("Values {} {}", v1, v2))
}

fn main() {
    App::new()
        .resource(r"/a/{v1}/{v2}/", |r| r.f(index))
        .finish();
}
// </minfo>
