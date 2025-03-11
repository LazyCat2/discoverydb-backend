#[macro_use]
extern crate rocket;
mod schemas;

use rocket::serde::json::Json;
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};
use schemas::*;
use sqlx::FromRow;

#[derive(Database)]
#[database("main")]
struct MainDB(sqlx::SqlitePool);

type Main = Connection<MainDB>;

#[get("/server/<id>")]
async fn fetch_server(mut db: Main, id: String) -> Option<Json<Vec<Server>>> {
    if let Ok(items) = sqlx::query("SELECT * FROM server WHERE id = ?")
        .bind(id)
        .fetch_all(&mut **db)
        .await
    {
        Some(Json(
            items
                .iter()
                .filter_map(|row| Server::from_row(row).ok())
                .collect::<Vec<Server>>(),
        ))
    } else {
        None
    }
}

#[get("/bot/<id>")]
async fn fetch_bot(mut db: Main, id: String) -> Option<Json<Vec<Bot>>> {
    if let Ok(items) = sqlx::query("SELECT * FROM bot WHERE id = ?")
        .bind(id)
        .fetch_all(&mut **db)
        .await
    {
        Some(Json(
            items
                .iter()
                .filter_map(|row| Bot::from_row(row).ok())
                .collect::<Vec<Bot>>(),
        ))
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(MainDB::init())
        .mount("/", routes![fetch_server, fetch_bot])
}
