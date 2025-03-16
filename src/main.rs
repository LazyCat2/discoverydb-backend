#[macro_use]
extern crate rocket;
mod create;
mod fetch;
mod macros;
mod result;
mod schemas;

use crate::create::make_test_server;
use crate::result::Error;
use crate::schemas::Server;
use fetch::*;
use rocket::serde::json::Json;
use rocket::State;

pub type DB = State<schemas::Database>;
pub type APIResponce<T> = Result<Json<T>, Error>;

#[launch]
fn rocket() -> _ {
    rocket::build().manage(schemas::Database::init()).mount(
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
            // Create
            make_test_server,
        ],
    )
}
