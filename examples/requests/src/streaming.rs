// <streaming>
use actix_web::{error, web, Error, HttpResponse};
use futures::{future::result, Future, Stream};

pub fn index(payload: web::Payload) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    Box::new(
        payload
            .from_err()
            .fold((), |_, chunk| {
                println!("Chunk: {:?}", chunk);
                result::<_, error::PayloadError>(Ok(()))
            })
            .map(|_| HttpResponse::Ok().into()),
    )
}
// </streaming>

pub fn main() {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().route("/", web::post().to_async(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
