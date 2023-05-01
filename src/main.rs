mod config;
mod database;
// mod api_server;
pub mod paths;


// fn main() {
//     println!("MCMF database!");
    
//     paths::create_required_directories();
//     database::create_members_table();
    
//     // inserting a record into the table
//     let last_inserted_row_id: i64;
//     match database::insert_member("name", "address", "phone", "email", "parish", "ecclesiastical_district", "comments") {
//         Ok(id) => {
//             last_inserted_row_id = id;
//             println!("Successfully added the member and last row_id is {}", id);
//         },
//         Err(e) => {
//             last_inserted_row_id = -9999;
//             println!("Error in adding the new member: {}", e);
//         }
//     };

//     println!("Inserted new user with ID {}", last_inserted_row_id);
// }
use actix_web::{web, App, HttpServer, HttpResponse};
// use sqlx::{SqlitePool};
use sqlx::Pool;
use sqlx::sqlite::Sqlite;


#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let pool = Pool::<Sqlite>::connect("assets/sqlite.db").await.unwrap();
    // let pool = SqlitePool::new("assets/sqlite.db").await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL
    )").execute(&pool.clone()).await.unwrap();

    let name = "Santo";
    let age = 35;
    
    let result = sqlx::query(&format!("INSERT INTO people (name, age) VALUES ({}, {})", name, age))
        .execute(&pool.clone())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Successfully inserted into database"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to insert into database"),
    };

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(pool.clone())
    //         .service(web::resource("/users").route(web::post().to(api_server::add_user)))
    // });
    Ok(())

}
