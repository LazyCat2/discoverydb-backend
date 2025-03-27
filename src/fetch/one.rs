use crate::{create_error, fetch_from_db, DB};
use crate::{schemas::*, APIResponce};
use rocket::serde::json::Json;

#[get("/server/<id>")]
pub async fn fetch_server(db: &DB, id: String) -> APIResponce<Server> {
    fetch_from_db!(one db.server, id)
}

#[get("/bot/<id>")]
pub async fn fetch_bot(db: &DB, id: String) -> APIResponce<Bot> {
    fetch_from_db!(one db.bot, id)
}

#[get("/client/<slug>")]
pub async fn fetch_client(db: &DB, slug: String) -> APIResponce<Client> {
    fetch_from_db!(one db.client, slug)
}

#[get("/theme/<slug>")]
pub async fn fetch_theme(db: &DB, slug: String) -> APIResponce<Theme> {
    fetch_from_db!(one db.theme, slug)
}

#[get("/plugin/<slug>")]
pub async fn fetch_plugin(db: &DB, slug: String) -> APIResponce<Plugin> {
    fetch_from_db!(one db.plugin, slug)
}
