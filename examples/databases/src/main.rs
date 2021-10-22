// <handler>
fn insert_new_user(db: &SqliteConnection, user: CreateUser) -> Result<User, Error> {
    use self::schema::users::dsl::*;

    // Create insertion model
    let uuid = format!("{}", uuid::Uuid::new_v4());
    let new_user = models::NewUser {
        id: &uuid,
        name: &user.name,
    };

    // normal diesel operations
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&self.0)
        .expect("Error inserting person");

    let mut items = users
        .filter(id.eq(&uuid))
        .load::<models::User>(&self.0)
        .expect("Error loading person");

    Ok(items.pop().unwrap())
}
// </handler>

// <main>
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Create connection pool
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start HTTP server
    HttpServer::new(move || {
        App::new().data(pool.clone())
            .resource("/{name}", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </main>

// <index>
async fn index(pool: web::Data<DbPool>, name: web::Path<(String)>) -> impl Responder {
    let name = name.into_inner();

    let conn = pool.get().expect("couldn't get db connection from pool");

    let user = web::block(move || actions::insert_new_user(&conn, &user))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    
    Ok(HttpResponse::Ok().json(user))
}
// </index>
