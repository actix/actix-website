extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod cfg;
mod dhandler;
mod minfo;
mod norm;
mod norm2;
mod path;
mod path2;
mod pbuf;
mod pred;
mod pred2;
mod prefix;
mod resource;
mod scope;
mod url_ext;
mod urls;

// <main>
use actix_web::{http::Method, App, HttpRequest, HttpResponse};

fn index(req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

fn main() {
    App::new()
        .route("/user/{name}", Method::GET, index)
        .route("/user/{name}", Method::POST, index)
        .finish();
}
// </main>
