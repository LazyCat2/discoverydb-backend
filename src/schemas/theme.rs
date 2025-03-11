use serde::{Deserialize, Serialize};

use super::Whatever;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Theme {
    name: String,
    description: Option<String>,
    author: String,
    data: Whatever,
}
