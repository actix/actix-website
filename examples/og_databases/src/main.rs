// <actor>
// use actix::prelude::*;

// struct DbExecutor(SqliteConnection);

// impl Actor for DbExecutor {
//     type Context = SyncContext<Self>;
// }
// </actor>

// <message>
// struct CreateUser {
//     name: String,
// }

// impl Message for CreateUser {
//     type Result = Result<User, Error>;
// }
// </message>

// <handler>
// impl Handler<CreateUser> for DbExecutor {
//     type Result = Result<User, Error>;

//     fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
//         use self::schema::users::dsl::*;

//         // Create insertion model
//         let uuid = format!("{}", uuid::Uuid::new_v4());
//         let new_user = models::NewUser {
//             id: &uuid,
//             name: &msg.name,
//         };

//         // normal diesel operations
//         diesel::insert_into(users)
//             .values(&new_user)
//             .execute(&self.0)
//             .expect("Error inserting person");

//         let mut items = users
//             .filter(id.eq(&uuid))
//             .load::<models::User>(&self.0)
//             .expect("Error loading person");

//         Ok(items.pop().unwrap())
//     }
// }
// </handler>

// <main>
// /// This is state where we will store *DbExecutor* address.
// struct State {
//     db: Addr<DbExecutor>,
// }

// fn main() {
//     let sys = actix::System::new("diesel-example");

//     // Start 3 parallel db executors
//     let addr = SyncArbiter::start(3, || {
//         DbExecutor(SqliteConnection::establish("test.db").unwrap())
//     });

//     // Start http server
//     HttpServer::new(move || {
//         App::with_state(State { db: addr.clone() })
//             .resource("/{name}", |r| r.method(Method::GET).a(index))
//     })
//     .bind("127.0.0.1:8080")
//     .unwrap()
//     .start()
//     .unwrap();

//     println!("Started http server: 127.0.0.1:8080");
//     let _ = sys.run();
// }
// </main>

// <index>
// /// Async handler
// fn index(req: &HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
//     let name = &req.match_info()["name"];

//     // Send message to `DbExecutor` actor
//     req.state()
//         .db
//         .send(CreateUser {
//             name: name.to_owned(),
//         })
//         .from_err()
//         .and_then(|res| match res {
//             Ok(user) => Ok(HttpResponse::Ok().json(user)),
//             Err(_) => Ok(HttpResponse::InternalServerError().into()),
//         })
//         .responder()
// }
// </index>
