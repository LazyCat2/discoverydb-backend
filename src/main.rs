#[macro_use]
extern crate rocket;
mod fetch;
mod macros;
mod result;
mod schemas;

use fetch::*;
use rocket::State;

pub type DB = State<schemas::Database>;

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
        ],
    )
}
