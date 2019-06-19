// <example>
use actix_web::{http, HttpRequest, HttpResponse};

pub fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .connection_type(http::ConnectionType::Close) // <- Close connection
        .force_close() // <- Alternative method
        .finish()
}
// </example>
