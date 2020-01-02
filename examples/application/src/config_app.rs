// <config>
use actix_service::ServiceFactory;
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};
use actix_web::{web, App, Error, HttpResponse};

fn create_app() -> App<
    impl ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
    >,
    impl MessageBody,
> {
    App::new().service(
        web::scope("/app")
            .route("/index.html", web::get().to(|| HttpResponse::Ok())),
    )
}
// </config>
