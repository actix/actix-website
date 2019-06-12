use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Event {
    id: Option<i32>,
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

fn store_event_in_db(timestamp: f64, kind: String, tags: Vec<String>) -> Event {
    // store item in db and get new_event
    // use id to lookup item
    Event {
        id: Some(1),
        timestamp: timestamp,
        kind: kind,
        tags: tags,
    }
}

fn capture_event(evt: web::Json<Event>) -> actix_web::Result<HttpResponse> {
    let new_event = store_event_in_db(evt.timestamp, evt.kind.clone(), evt.tags.clone());
    Ok(HttpResponse::Ok().json(new_event))
}

fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/event", web::post().to(capture_event))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
