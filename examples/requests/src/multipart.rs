// <multipart>
// use actix_web::{error, Error, HttpRequest, HttpResponse};
// use futures::Future;

// pub fn index(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = Error>> {
//     // get multipart and iterate over multipart items
//     req.multipart().and_then(|item| match item {
//         multipart::MultipartItem::Field(field) => {
//             println!(
//                 "==== FIELD ==== {:?} {:?}",
//                 field.headers(),
//                 field.content_type()
//             );
//             Either::A(
//                 field
//                     .map(|chunk| {
//                         println!("-- CHUNK: \n{}", std::str::from_utf8(&chunk).unwrap());
//                     })
//                     .fold((), |_, _| result(Ok(()))),
//             )
//         }
//         multipart::MultipartItem::Nested(mp) => Either::B(result(Ok(()))),
//     })
// }
// </multipart>
