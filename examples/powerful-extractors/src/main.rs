use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// <powerful-extractors>
#[derive(Deserialize, Serialize)]
struct Event {
    id: Option<i32>,
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

async fn capture_event(evt: web::Json<Event>) -> impl Responder {
    let new_event = evt.save();
    format!("got event {}", new_event.id.unwrap())
}
// </powerful-extractors>

impl Event {
    fn save(&self) -> Event {
        // store item in db and get new_event
        // use id to lookup item
        Event {
            id: Some(1),
            timestamp: self.timestamp,
            kind: self.kind.to_string(),
            tags: self.tags.to_vec(),
        }
    }
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/event", web::post().to(capture_event))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
