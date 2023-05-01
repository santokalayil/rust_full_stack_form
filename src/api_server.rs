// use actix_web::{web, App, HttpServer};
// use rusqlite::{params, Connection};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    name: String,
    email: String,
}

async fn add_user_to_db(
    conn: &Connection,
    user: &User,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        params![user.name, user.email],
    )?;
    Ok(())
}

pub(crate) async fn add_user(
    pool: web::Data<SqlitePool>,
    user: web::Json<User>,
) -> Result<web::Json<User>, actix_web::Error> {
    let conn = pool.get().await.map_err(|e| {
        println!("Failed to get DB connection: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })?;

    add_user_to_db(&conn, &user)
        .await
        .map_err(|e| {
            println!("Failed to add user to DB: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to add user to DB")
        })?;

    Ok(user)
}


use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::sqlite::SqlitePool;

pub async fn insert_into_db(pool: web::Data<SqlitePool>, name: String, age: i32) -> impl Responder {
    let result = sqlx::query!("INSERT INTO people (name, age) VALUES (?, ?)", name, age)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Successfully inserted into database"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to insert into database"),
    }
}



