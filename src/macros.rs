#[macro_export]
macro_rules! fetch_from_db {
    ($collection:expr, $id:expr, id) => {{
        use mongodb::bson::doc;

        match $collection.find_one(doc! {"_id": $id}, None) {
            Err(error) => Err(create_error!(DatabaseError {
                error: error.to_string()
            })),

            Ok(item) => match item {
                Some(item) => Ok(Json(item)),
                None => Err(create_error!(NotFound)),
            },
        }
    }};

    ($collection:expr, $slug:expr, slug) => {{
        use mongodb::bson::doc;

        match $collection.find_one(doc! {"listing.slug": $slug}, None) {
            Err(error) => Err(create_error!(DatabaseError {
                error: error.to_string()
            })),

            Ok(item) => match item {
                Some(item) => Ok(Json(item)),
                None => Err(create_error!(NotFound)),
            },
        }
    }};

    ($collection:expr, $struct:ty) => {{
        use crate::schemas::Visibility;

        match $collection.find(None, None) {
            Err(error) => Err(create_error!(DatabaseError {
                error: error.to_string()
            })),

            Ok(mut cursor) => {
                let mut result: Vec<$struct> = Vec::new();
                while let Some(doc) = cursor.next() {
                    if let Ok(doc) = doc {
                        // check if visibility is public (doc is of type Plugin)
                        if(doc.listing.visibility == Visibility::Public) {
                            result.push(doc);
                        }
                    }
                }

                Ok(Json(result))
            }
        }
    }};
}
