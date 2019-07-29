// <wrap-fn>
use actix_service::Service;
use actix_web::{web, App};
use futures::future::Future;

fn main() {
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
            web::get().to(|| "Hello, middleware!"),
        );
}
// </wrap-fn>
