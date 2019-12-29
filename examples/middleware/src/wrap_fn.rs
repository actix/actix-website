// <wrap-fn>
use actix_service::Service;
use actix_web::{web, App};
use futures::future::FutureExt;

#[actix_rt::main]
async fn main() {
    let app = App::new()
        .wrap_fn(|req, srv| {
            println!("Hi from start. You requested: {}", req.path());
            srv.call(req).map(|res| {
                println!("Hi from response");
                res
            })
        })
        .route(
            "/index.html",
            web::get().to(|| async {
                "Hello, middleware!"
            }),
        );
}
// </wrap-fn>
