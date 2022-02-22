// <identity-two>
use actix_web::{
    get, http::header::ContentEncoding, middleware, App, HttpResponse, HttpServer,
};

static HELLO_WORLD: &[u8] = &[
    0x1f, 0x8b, 0x08, 0x00, 0xa2, 0x30, 0x10, 0x5c, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9,
    0x57, 0x28, 0xcf, 0x2f, 0xca, 0x49, 0xe1, 0x02, 0x00, 0x2d, 0x3b, 0x08, 0xaf, 0x0c, 0x00,
    0x00, 0x00,
];

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentEncoding::Gzip)
        .body(HELLO_WORLD)
}
// </identity-two>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
