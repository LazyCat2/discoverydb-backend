use crate::schemas::*;
use crate::{create_error, Main};
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use sqlx::FromRow;
use crate::result::Error;

#[get("/server/<id>")]
pub async fn fetch_server(mut db: Main, id: String) -> Result<Json<Server>, Error> {
    let item = sqlx::query("SELECT * FROM server WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map_err(|err| create_error!(DatabaseError {
            error: err.to_string()
        }))?;

    Server::from_row(&item)
        .map(Json)
        .map_err(|_| create_error!(InternalError))
}

#[get("/bot/<id>")]
pub async fn fetch_bot(mut db: Main, id: String) -> Result<Json<Bot>, Error> {
    let item = sqlx::query("SELECT * FROM bot WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map_err(|err| create_error!(DatabaseError {
            error: err.to_string()
        }))?;

    Bot::from_row(&item)
        .map(Json)
        .map_err(|_| create_error!(InternalError))
}

#[get("/client/<id>")]
pub async fn fetch_client(mut db: Main, id: String) -> Result<Json<Client>, Error> {
    let item = sqlx::query("SELECT * FROM client WHERE name = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map_err(|err| create_error!(DatabaseError {
            error: err.to_string()
        }))?;

    Client::from_row(&item)
        .map(Json)
        .map_err(|_| create_error!(InternalError))
}

#[get("/theme/<id>")]
pub async fn fetch_theme(mut db: Main, id: String) -> Result<Json<Theme>, Error> {
    let item = sqlx::query("SELECT * FROM theme WHERE name = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map_err(|err| create_error!(DatabaseError {
            error: err.to_string()
        }))?;

    Theme::from_row(&item)
        .map(Json)
        .map_err(|_| create_error!(InternalError))
}

#[get("/plugin/<id>")]
pub async fn fetch_plugin(mut db: Main, id: String) -> Result<Json<Plugin>, Error> {
    let item = sqlx::query("SELECT * FROM plugin WHERE name = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map_err(|err| create_error!(DatabaseError {
            error: err.to_string()
        }))?;

    Plugin::from_row(&item)
        .map(Json)
        .map_err(|_| create_error!(InternalError))
}