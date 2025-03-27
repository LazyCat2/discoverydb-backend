use crate::{create_error, fetch_from_db, DB};
use crate::{schemas::*, APIResponce};
use rocket::serde::json::Json;

#[get("/servers")]
pub async fn fetch_servers(db: &DB) -> APIResponce<Vec<Server>> {
    fetch_from_db!(all db.server, Server)
}

#[get("/bots")]
pub async fn fetch_bots(db: &DB) -> APIResponce<Vec<Bot>> {
    fetch_from_db!(all db.bot, Bot)
}

#[get("/clients")]
pub async fn fetch_clients(db: &DB) -> APIResponce<Vec<Client>> {
    fetch_from_db!(all db.client, Client)
}

#[get("/themes")]
pub async fn fetch_themes(db: &DB) -> APIResponce<Vec<Theme>> {
    fetch_from_db!(all db.theme, Theme)
}

#[get("/plugins")]
pub async fn fetch_plugins(db: &DB) -> APIResponce<Vec<Plugin>> {
    fetch_from_db!(all db.plugin, Plugin)
}
