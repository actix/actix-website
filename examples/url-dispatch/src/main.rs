pub mod cfg;
pub mod dhandler;
pub mod minfo;
pub mod norm;
pub mod norm2;
pub mod path;
pub mod path2;
pub mod pbuf;
pub mod pred;
pub mod pred2;
pub mod resource;
pub mod scope;
pub mod url_ext;
pub mod urls;

// <main>
use actix_web::{web, App, HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

fn main() {
    App::new()
        .route("/user/{name}", web::get().to(index))
        .route("/user/{name}", web::post().to(index));
}
// </main>
