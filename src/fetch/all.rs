use crate::schemas::*;
use crate::Main;
use rocket::serde::json::Json;
use rocket_db_pools::sqlx;
use sqlx::FromRow;

#[get("/servers")]
pub async fn fetch_servers(mut db: Main) -> Option<Json<Vec<Server>>> {
    if let Ok(items) = sqlx::query("SELECT * FROM server")
        .fetch_all(&mut **db)
        .await
    {
        Some(Json(
            items
                .iter()
                .filter_map(|row| Server::from_row(row).ok())
                .collect::<Vec<Server>>(),
        ))
    } else {
        None
    }
}

#[get("/bots")]
pub async fn fetch_bots(mut db: Main) -> Option<Json<Vec<Bot>>> {
    match sqlx::query("SELECT * FROM bot").fetch_all(&mut **db).await {
        Ok(items) => Some(Json(
            items
                .iter()
                .filter_map(|row| Bot::from_row(row).ok())
                .collect::<Vec<Bot>>(),
        )),

        Err(error) => {
            println!("{error}");
            None
        }
    }
}

#[get("/clients")]
pub async fn fetch_clients(mut db: Main) -> Option<Json<Vec<Client>>> {
    if let Ok(items) = sqlx::query("SELECT * FROM client")
        .fetch_all(&mut **db)
        .await
    {
        Some(Json(
            items
                .iter()
                .filter_map(|row| Client::from_row(row).ok())
                .collect::<Vec<Client>>(),
        ))
    } else {
        None
    }
}

#[get("/themes")]
pub async fn fetch_themes(mut db: Main) -> Option<Json<Vec<Theme>>> {
    if let Ok(items) = sqlx::query("SELECT * FROM theme")
        .fetch_all(&mut **db)
        .await
    {
        Some(Json(
            items
                .iter()
                .filter_map(|row| Theme::from_row(row).ok())
                .collect::<Vec<Theme>>(),
        ))
    } else {
        None
    }
}

#[get("/plugins")]
pub async fn fetch_plugins(mut db: Main) -> Option<Json<Vec<Plugin>>> {
    if let Ok(items) = sqlx::query("SELECT * FROM plugin")
        .fetch_all(&mut **db)
        .await
    {
        Some(Json(
            items
                .iter()
                .filter_map(|row| Plugin::from_row(row).ok())
                .collect::<Vec<Plugin>>(),
        ))
    } else {
        None
    }
}
