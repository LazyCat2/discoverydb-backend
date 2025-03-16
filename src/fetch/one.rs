use crate::result::Error;
use crate::schemas::*;
use crate::{create_error, fetch_from_db, DB};
use rocket::serde::json::Json;

#[get("/server/<id>")]
pub async fn fetch_server(db: &DB, id: String) -> Result<Json<Server>, Error> {
    fetch_from_db!(db.server, id, id)
}

#[get("/bot/<id>")]
pub async fn fetch_bot(db: &DB, id: String) -> Result<Json<Bot>, Error> {
    fetch_from_db!(db.bot, id, id)
}

#[get("/client/<slug>")]
pub async fn fetch_client(db: &DB, slug: String) -> Result<Json<Client>, Error> {
    fetch_from_db!(db.client, slug, slug)
}

#[get("/theme/<slug>")]
pub async fn fetch_theme(db: &DB, slug: String) -> Result<Json<Theme>, Error> {
    fetch_from_db!(db.theme, slug, slug)
}

#[get("/plugin/<slug>")]
pub async fn fetch_plugin(db: &DB, slug: String) -> Result<Json<Plugin>, Error> {
    fetch_from_db!(db.plugin, slug, slug)
}
