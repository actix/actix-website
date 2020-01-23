// <streaming>
use actix_web::{web, Error, HttpResponse};
use futures::StreamExt;

async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    println!("Chunk: {:?}", bytes);
    Ok(HttpResponse::Ok().finish())
}
// </streaming>

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
