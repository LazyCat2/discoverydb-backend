use crate::result::Error;
use crate::schemas::*;
use crate::{create_error, fetch_public_items, DB};
use rocket::serde::json::Json;

#[get("/server/<id>")]
pub async fn fetch_server(db: &DB, id: String) -> Result<Json<Server>, Error> {
	fetch_public_items!(server, id)
}

#[get("/bot/<id>")]
pub async fn fetch_bot(db: &DB, id: String) -> Result<Json<Bot>, Error> {
	fetch_public_items!(bot, id)
}

#[get("/client/<name>")]
pub async fn fetch_client(db: &DB, name: String) -> Result<Json<Client>, Error> {
	fetch_public_items!(client, name)
}

#[get("/theme/<name>")]
pub async fn fetch_theme(db: &DB, name: String) -> Result<Json<Theme>, Error> {
	fetch_public_items!(theme, id)
}

#[get("/plugin/<name>")]
pub async fn fetch_plugin(db: &DB, name: String) -> Result<Json<Plugin>, Error> {
	fetch_public_items!(plugin, name)
}
