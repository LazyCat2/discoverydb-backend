use serde::{Deserialize, Serialize};
use crate::schemas::Visibility;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Plugin {
    pub name: String,
    pub description: Option<String>,
    pub developer: String,
    pub source: String,

    pub visibility: Visibility,
}
