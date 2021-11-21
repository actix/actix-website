pub mod default_headers;
pub mod errorhandler;
pub mod logger;
pub mod user_sessions;
pub mod wrap_fn;

// <simple>
use std::pin::Pin;
use std::task::{Context, Poll};

//Database pool. This sqlite sqlx pool.
use sqlx::{Connection, SqlitePool, sqlite};

use actix_web::dev::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct TokenAuthMiddlewareInit{
    //All saved vars need impl trait Clone
    some_data: String,
    database: Pool<Sqlite>
}

impl TokenAuthMiddlewareInit{
    pub fn new(income_data: String, db: Pool<Sqlite>) -> Self{
        TokenAuthMiddlewareInit{
            some_data: income_data,
            database: db
        }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for TokenAuthMiddlewareInit
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TokenAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(TokenAuthMiddleware { 
            service:service, 
            some_data: self.some_data.clone(), 
            database: self.db.clone() 
        })
    }
}

pub struct TokenAuthMiddleware<S> {
    service: S,
    some_data: String,
    database: Pool<Sqlite>
}

impl<S, B> Service for TokenAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        
        //This you can use saved vars: some_data and database.
        if self.some_data.as_str() == "disable_auth"{
            //End connection from middleware
            return Box::pin(async {
                Err(actix_web::Error::from(HttpResponse::Ok().status(StatusCode::UNAUTHORIZED).body("UNAUTHORIZED")))
            });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}
// </simple>

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    let db = SqlitePool::connect("sqlite::memory:").await?;
    let static_string = String::from("some_string_data");

    HttpServer::new(|| {
        App::new()
            .wrap(TokenAuthMiddlewareInit::new(static_string.clone(), db.clone()))
            .service(
                web::resource("/")
                    .to(|| async {
                        "Hello, middleware! Check the console where the server is run."
                    }),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
