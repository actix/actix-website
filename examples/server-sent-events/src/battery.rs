use actix_web::{Responder, rt};
use actix_web_lab::sse;
use tokio::sync::mpsc;

// <battery>
pub async fn events() -> impl Responder {
    let (tx, rx) = mpsc::channel(10);

    rt::spawn(async move {
        let event = sse::Data::new("Connected!").event("ready");
        tx.send(event.into()).await
    });

    sse::Sse::from_infallible_receiver(rx)
}
// </battery>
