// <streaming>
// use actix_web::{error, web, Error, HttpResponse};
// use futures::{future::result, Future, Stream};

// pub fn index(payload: web::Payload) -> Box<Future<Item = HttpResponse, Error = Error>> {
//     payload
//         .from_err()
//         .fold((), |_, chunk| {
//             println!("Chunk: {:?}", chunk);
//             result::<_, error::PayloadError>(Ok(()))
//         })
//         .map(|_| HttpResponse::Ok().finish())
//         .responder()
// }
// </streaming>
