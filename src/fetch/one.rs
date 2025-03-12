use crate::schemas::*;
use crate::Main;
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use sqlx::FromRow;

#[get("/server/<id>")]
pub async fn fetch_server(mut db: Main, id: String) -> Option<Json<Server>> {
    if let Ok(item) = sqlx::query("SELECT * FROM server WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        if let Ok(item) = Server::from_row(&item) {
            Some(Json(item))
        } else {
            None
        }
    } else {
        None
    }
}

#[get("/bot/<id>")]
pub async fn fetch_bot(mut db: Main, id: String) -> Option<Json<Bot>> {
    if let Ok(item) = sqlx::query("SELECT * FROM bot WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        if let Ok(item) = Bot::from_row(&item) {
            Some(Json(item))
        } else {
            None
        }
    } else {
        None
    }
}

#[get("/client/<id>")]
pub async fn fetch_client(mut db: Main, id: String) -> Option<Json<Client>> {
    if let Ok(item) = sqlx::query("SELECT * FROM client WHERE name = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        if let Ok(item) = Client::from_row(&item) {
            Some(Json(item))
        } else {
            None
        }
    } else {
        None
    }
}

#[get("/theme/<id>")]
pub async fn fetch_theme(mut db: Main, id: String) -> Option<Json<Theme>> {
    if let Ok(item) = sqlx::query("SELECT * FROM theme WHERE name = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        if let Ok(item) = Theme::from_row(&item) {
            Some(Json(item))
        } else {
            None
        }
    } else {
        None
    }
}

#[get("/plugin/<id>")]
pub async fn fetch_plugin(mut db: Main, id: String) -> Option<Json<Plugin>> {
    if let Ok(item) = sqlx::query("SELECT * FROM plugin WHERE name = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        if let Ok(item) = Plugin::from_row(&item) {
            Some(Json(item))
        } else {
            None
        }
    } else {
        None
    }
}
