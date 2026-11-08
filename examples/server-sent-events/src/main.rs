pub mod battery;

// <server-sent-events>
use std::time::Duration;

use actix_web::{App, HttpServer, Responder, rt, web};
use actix_web_lab::sse;
use tokio::sync::mpsc;

async fn events() -> impl Responder {
    let (tx, rx) = mpsc::channel(10);

    rt::spawn(async move {
        let mut interval = rt::time::interval(Duration::from_secs(5));

        loop {
            interval.tick().await;

            if tx
                .send(sse::Data::new("Hello from Actix Web!").into())
                .await
                .is_err()
            {
                break;
            }
        }
    });

    sse::Sse::from_infallible_receiver(rx)
        .with_keep_alive(Duration::from_secs(15))
        .with_retry_duration(Duration::from_secs(3))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/events", web::get().to(events)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </server-sent-events>
