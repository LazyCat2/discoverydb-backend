#[macro_export]
macro_rules! fetch_public_items {
    ($collection:expr, id) => {{
        match db.$collection.find_one(doc! {"_id": id}, None) {
            Err(error) => create_error!(DatabaseError {
                error: err.to_string()
            }),
            Ok(item) => Ok(Json(
            	item
            ))
        }
    }};

    ($collection:expr, name) => {{
    	match db.$collection.find_one(doc! {"name": name}, None) {
    	    Err(error) => create_error!(DatabaseError {
    	        error: err.to_string()
    	    }),
    	    Ok(item) => Ok(Json(
    	    	item
    	    ))
    	}
    }};

    ($collection:expr, $struct:ty) => {{
    	match db.$collection.find(None, None) {
    	    Err(error) => create_error!(DatabaseError {
    	        error: err.to_string()
    	    }),
    	    Ok(item) => Ok(Json(
    	    	item.collect::Vec<$struct>().await
    	    ))
    	}
    }};
}
