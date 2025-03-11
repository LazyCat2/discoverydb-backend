use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Plugin {
    pub name: String,
    pub description: Option<String>,
    pub developer: String,
    pub source: String,
}
