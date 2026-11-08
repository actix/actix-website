use actix_web::{Responder, web};
use serde::Deserialize;

// <query>
#[derive(Deserialize)]
pub struct EventFilter {
    kind: String,
    #[serde(default)]
    page: u32,
}

// GET /events?kind=click&page=2
#[rustfmt::skip]
pub async fn list_events(
    filter: web::Query<EventFilter>,
) -> impl Responder {
    format!("Events of kind {} on page {}", filter.kind, filter.page)
}
// </query>
