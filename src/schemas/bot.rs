use serde::{Deserialize, Serialize};
use crate::schemas::Visibility;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Bot {
    id: String,
    name: String,
    about: Option<String>,
    avatar: Option<String>,
    banner: Option<String>,
    developer: String,

    pub visibility: Visibility,
}
