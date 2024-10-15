#![allow(dead_code, unused_variables)]

// <from-fn>
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    App, Error,
};

async fn my_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    next.call(req).await
    // post-processing
}

#[actix_web::main]
async fn main() {
    let app = App::new().wrap(from_fn(my_middleware));
}
// </from-fn>
