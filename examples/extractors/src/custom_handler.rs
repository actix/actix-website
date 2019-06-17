use actix_web::{web, HttpRequest, HttpResponse};

struct MyHandler {}
struct MyInfo {}

// <custom-handler>
impl<S> Handler<S> for MyHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let params = web::Path::<(String, String)>::extract(req);
        let info = web::Json::<MyInfo>::extract(req);

        HttpResponse::Ok().into()
    }
}
// </custom-handler>
