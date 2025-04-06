use std::io;

use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};
use serde::Serialize;
use uuid::Uuid;

// <handler>
// Importing the existing entity modules
pub mod prelude;
pub mod users;
use crate::users::ActiveModel as UserActiveModel;

#[derive(Debug, Serialize)]
struct User {
    id: String,
    name: String,
}

async fn insert_new_user(
    conn: &DatabaseConnection,
    user_name: String,
) -> Result<User, sea_orm::DbErr> {
    // Create insertion model
    let uid = Uuid::new_v4().to_string();
    let new_user = UserActiveModel {
        id: Set(uid.clone()),
        name: Set(user_name),
        ..Default::default()
    };

    // Insert the user
    let user = new_user.insert(conn).await?;

    Ok(User {
        id: user.id,
        name: user.name,
    })
}
// </handler>

// <main>
#[actix_web::main]
async fn main() -> io::Result<()> {
    let database_url = "sqlite:app.db";
    let conn = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .route("/{name}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </main>

// <index>
async fn index(
    conn: web::Data<DatabaseConnection>,
    name: web::Path<(String,)>,
) -> actix_web::Result<impl Responder> {
    let (name,) = name.into_inner();

    let user = insert_new_user(&conn, name)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}
// </index>
