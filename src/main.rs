#[macro_use]
extern crate rocket;
mod fetch;
mod schemas;
mod result;
mod macros;

use fetch::*;
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("main")]
pub struct MainDB(sqlx::SqlitePool);

pub type Main = Connection<MainDB>;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(MainDB::init()).mount(
        "/",
        routes![
            // Single
            fetch_server,
            fetch_bot,
            fetch_client,
            fetch_theme,
            fetch_plugin,

            // Multiple
            fetch_servers,
            fetch_bots,
            fetch_clients,
            fetch_themes,
            fetch_plugins,
        ],
    )
}
