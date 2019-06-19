use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Event {
    id: Option<i32>,
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

fn store_in_db(timestamp: f64, kind: &String, tags: &Vec<String>) -> Event {
    // store item in db and get new_event
    // use id to lookup item
    Event {
        id: Some(1),
        timestamp: timestamp,
        kind: kind.to_string(),
        tags: tags.to_vec(),
    }
}

fn capture_event(evt: web::Json<Event>) -> impl Responder {
    let new_event = store_in_db(evt.timestamp, &evt.kind, &evt.tags);
    format!("got event {}", new_event.id.unwrap())
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
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
