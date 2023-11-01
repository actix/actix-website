#[get("/")]
async fn hello_world() -> impl actix_web::Responder {
    "hello world"
}

// <shuttle-hello-world>
use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        // set up your service here, e.g.:
        cfg.service(hello_world);
    };

    Ok(config.into())
}
// </shuttle-hello-world>
