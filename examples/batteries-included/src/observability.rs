use actix_web::{App, HttpResponse, HttpServer, get};
use actix_web_lab::extract::Path;
use tracing_actix_web::{RequestId, TracingLogger};

// <observability>
#[rustfmt::skip]
#[get("/accounts/{account_id}/status")]
#[tracing::instrument(skip_all, fields(%account_id))]
async fn status(
    Path(account_id): Path<u64>,
    req_id: RequestId,
) -> HttpResponse {
    // Logged as child of request trace and includes the account ID.
    tracing::info!("Account status requested");

    HttpResponse::Ok()
        .insert_header(("x-request-id", req_id.to_string()))
        .body("Ready")
}
// </observability>

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(|| App::new().wrap(TracingLogger::default()).service(status))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
