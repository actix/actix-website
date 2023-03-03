#![allow(dead_code)]

// <example>
use actix_web::{http, HttpRequest, HttpResponse};

async fn index(_req: HttpRequest) -> HttpResponse {
    let mut resp = HttpResponse::Ok()
        .force_close() // <- Close connection on HttpResponseBuilder
        .finish();

    // Alternatively close connection on the HttpResponse struct
    resp.head_mut().set_connection_type(http::ConnectionType::Close);

    resp
}
// </example>

// ConnectionType::Close
// ConnectionType::KeepAlive
// ConnectionType::Upgrade
