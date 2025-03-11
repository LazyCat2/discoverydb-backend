use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Bot {
    id: String,
    name: String,
    about: Option<String>,
    avatar: Option<String>,
    banner: Option<String>,
    developer: String,
}
