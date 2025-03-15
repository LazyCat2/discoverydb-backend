#[macro_export]
macro_rules! fetch_public_items {
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

    ($collection:expr, $name:expr, name) => {{
        use mongodb::bson::doc;

        match $collection.find_one(doc! {"name": $name}, None) {
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
        match $collection.find(None, None) {
            Err(error) => Err(create_error!(DatabaseError {
                error: error.to_string()
            })),

            Ok(mut cursor) => {
                let mut result: Vec<$struct> = Vec::new();
                while let Some(doc) = cursor.next() {
                    if let Ok(doc) = doc {
                        result.push(doc);
                    }
                }

                Ok(Json(result))
            }
        }
    }};
}
