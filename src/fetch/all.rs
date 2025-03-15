use crate::schemas::*;
use crate::{create_error, fetch_public_items, Main};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use sqlx::FromRow;
use crate::result::Error;

#[get("/servers")]
pub async fn fetch_servers(mut db: Main) -> Result<Json<Vec<Server>>, Error> {
    fetch_public_items!(db, "server", Server)
}

#[get("/bots")]
pub async fn fetch_bots(mut db: Main) -> Result<Json<Vec<Bot>>, Error> {
    fetch_public_items!(db, "bot", Bot)
}

#[get("/clients")]
pub async fn fetch_clients(mut db: Main) -> Result<Json<Vec<Client>>, Error> {
    fetch_public_items!(db, "client", Client)
}

#[get("/themes")]
pub async fn fetch_themes(mut db: Main) -> Result<Json<Vec<Theme>>, Error> {
    fetch_public_items!(db, "theme", Theme)
}

#[get("/plugins")]
pub async fn fetch_plugins(mut db: Main) -> Result<Json<Vec<Plugin>>, Error> {
    fetch_public_items!(db, "plugin", Plugin)
}