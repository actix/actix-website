mod auth;
mod headers;
mod multipart;
mod query;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

struct DatabaseHandle;

// <powerful-extractors>
#[derive(Deserialize, Serialize)]
struct Event {
    id: i32,
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

#[rustfmt::skip]
async fn capture_event(
    db: web::Data<DatabaseHandle>,
    event: web::Json<Event>,
) -> impl Responder {
    let new_event = store_in_db(&db, &event).await;
    format!("Received event id={}", new_event.id)
}
// </powerful-extractors>

async fn store_in_db(_db: &DatabaseHandle, ev: &Event) -> Event {
    // store item in db and get new_event
    // use id to lookup item
    Event {
        id: 1,
        timestamp: ev.timestamp,
        kind: ev.kind.to_string(),
        tags: ev.tags.to_vec(),
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
            .route("/profile", web::get().to(auth::profile))
            .route("/events", web::get().to(query::list_events))
            .route(
                "/preferred-format",
                web::get().to(headers::preferred_format),
            )
            .route("/upload", web::post().to(multipart::upload))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web};

    use super::{headers, multipart, query};

    #[actix_web::test]
    async fn query_extracts_fields_and_defaults_page() {
        let app =
            test::init_service(App::new().route("/events", web::get().to(query::list_events)))
                .await;

        for (uri, expected) in [
            (
                "/events?kind=click&page=2",
                "Events of kind click on page 2",
            ),
            ("/events?kind=click", "Events of kind click on page 0"),
        ] {
            let request = test::TestRequest::get().uri(uri).to_request();
            let body = test::call_and_read_body(&app, request).await;

            assert_eq!(body, expected);
        }
    }

    #[actix_web::test]
    async fn accept_selects_the_highest_quality() {
        let app =
            test::init_service(App::new().route("/", web::get().to(headers::preferred_format)))
                .await;

        for (accept, expected) in [
            ("text/csv;q=0.5, application/json", r#"[["click",42]]"#),
            ("application/json;q=0.2, text/csv", "kind,count\nclick,42\n"),
        ] {
            let request = test::TestRequest::get()
                .insert_header(("accept", accept))
                .to_request();
            let response = test::call_service(&app, request).await;

            assert!(response.status().is_success());
            assert_eq!(test::read_body(response).await, expected);
        }
    }

    #[actix_web::test]
    async fn multipart_extracts_text_and_file() {
        let app =
            test::init_service(App::new().route("/", web::post().to(multipart::upload))).await;
        let payload = concat!(
            "--boundary\r\n",
            "Content-Disposition: form-data; name=\"title\"\r\n\r\n",
            "Report\r\n",
            "--boundary\r\n",
            "Content-Disposition: form-data; name=\"file\"; filename=\"report.txt\"\r\n",
            "Content-Type: text/plain\r\n\r\n",
            "hello\r\n",
            "--boundary--\r\n",
        );
        let request = test::TestRequest::post()
            .insert_header(("content-type", "multipart/form-data; boundary=boundary"))
            .set_payload(payload)
            .to_request();
        let body = test::call_and_read_body(&app, request).await;

        assert_eq!(body, "Received Report (5 bytes)");
    }
}
