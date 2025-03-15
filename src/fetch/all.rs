use crate::result::Error;
use crate::schemas::*;
use crate::{create_error, fetch_public_items, DB};
use rocket::serde::json::Json;

#[get("/servers")]
pub async fn fetch_servers(db: &DB) -> Result<Json<Vec<Server>>, Error> {
    fetch_public_items!(db.server, Server)
}

#[get("/bots")]
pub async fn fetch_bots(db: &DB) -> Result<Json<Vec<Bot>>, Error> {
    fetch_public_items!(db.bot, Bot)
}

#[get("/clients")]
pub async fn fetch_clients(db: &DB) -> Result<Json<Vec<Client>>, Error> {
    fetch_public_items!(db.client, Client)
}

#[get("/themes")]
pub async fn fetch_themes(db: &DB) -> Result<Json<Vec<Theme>>, Error> {
    fetch_public_items!(db.theme, Theme)
}

#[get("/plugins")]
pub async fn fetch_plugins(db: &DB) -> Result<Json<Vec<Plugin>>, Error> {
    fetch_public_items!(db.plugin, Plugin)
}
