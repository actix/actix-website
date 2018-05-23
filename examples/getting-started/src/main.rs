// <setup>
extern crate actix_web;
use actix_web::{HttpRequest, App, server};

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}
// </setup>
// <main>
fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
    })
       .bind("127.0.0.1:8088").unwrap()
       .run();
}
// </main>
