use mongodb::bson::{doc, Timestamp};
use rocket::serde::json::Json;
use rocket::time::macros::time;
use rocket::yansi::Paint;
use crate::{create_error, fetch_from_db, DB};
use crate::result::Error;
use crate::schemas::{Listing, Server, Visibility};

#[post("/testServer")]
pub async fn make_test_server(db: &DB) -> Result<Json<Server>, Error> {
    let server = Server {
        listing: Listing {
            id: Some("server id goes here".to_string()),
            slug: Some("haiii! :3 slug is not needed for servers".to_string()),
            name: "Revolt".to_string(),
            description: None,
            visibility: Visibility::Public,
            tags: vec!["average".to_string(), "discord mod".to_string()],
        },
        icon: None,
        banner: None,
        owner: "user id goes here".to_string(),
        invite: "rvlt.gg/lounge".to_string(),
        members: 69_420,
    };

    match db.server.insert_one(server, None) {
        Ok(insert_result) => {
            let inserted_id = insert_result.inserted_id.as_str().unwrap().to_string();

            match db.server.find_one(doc! {"_id": inserted_id}, None) {
                Ok(Some(inserted_server)) => Ok(Json(inserted_server)),
                Ok(None) => Err(create_error!(NotFound)),
                Err(error) => Err(create_error!(DatabaseError {
                    error: error.to_string()
                })),
            }
        }
        Err(error) => Err(create_error!(DatabaseError {
            error: error.to_string()
        })),
    }
}