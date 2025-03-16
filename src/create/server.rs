use crate::schemas::{Listing, Server, Visibility};
use crate::{create_error, APIResponce, DB};
use mongodb::bson::doc;
use rocket::serde::json::Json;

#[post("/testServer")]
pub async fn make_test_server(db: &DB) -> APIResponce<Server> {
    let mut server = Server {
        listing: Listing {
            id: Some("hallo".to_string()),
            name: "Revolt".to_string(),
            description: None,
            visibility: Visibility::Public,
            tags: vec!["average".to_string(), "discord mod".to_string()],
        },
        icon: None,
        banner: None,
        owner: "Me".to_string(),
        invite: "lounge".to_string(),
        members: 69_420,
    };

    match db.server.insert_one(server.clone(), None) {
        Ok(insert_result) => {
            let inserted_id = insert_result.inserted_id.as_str().unwrap().to_string();
            server.listing.id = Some(inserted_id);

            Ok(Json(server))
        }
        Err(error) => Err(create_error!(DatabaseError {
            error: error.to_string()
        })),
    }
}
