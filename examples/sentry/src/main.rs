// <middleware>
// use actix_web::{web, App, HttpResponse};
// use sentry;
// use sentry_actix::SentryMiddleware;

// use std::env;

// fn main() {
//     sentry::init("SENTRY_DSN_GOES_HERE");
//     env::set_var("RUST_BACKTRACE", "1");
//     sentry::integrations::panic::register_panic_handler();

//     let mut app = App::new()
//         // .data(state)
//         .wrap(SentryMiddleware::new())
//         .route("/", web::get().to(|| HttpResponse::Ok()));
// }
// </middleware>

// <hub>
// use sentry::{Hub, Level};
// use sentry_actix::ActixWebHubExt;

// let hub = Hub::from_request(req);
// hub.capture_message("Something is not well", Level::Warning);
// </hub>

// <hub2>
// use sentry::{Hub, Level};
// use sentry_actix::ActixWebHubExt;

// let hub = Hub::from_request(req);
// Hub::run(hub, || {
//     sentry::capture_message("Something is not well", Level::Warning);
// });
// </hub2>
