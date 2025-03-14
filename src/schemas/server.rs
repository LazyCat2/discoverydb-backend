use serde::{Deserialize, Serialize};
use crate::schemas::Visibility;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub owner: String,
    pub members: u32,
    pub invite: String,

    pub visibility: Visibility,
}
