use serde::{Deserialize, Serialize};
use crate::schemas::Visibility;

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[repr(u8)]
pub enum ClientPlatform {
    Android,
    iOS,
    Web,
    Desktop,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Client {
    pub id: u16,
    pub name: String,
    pub developer: String,
    pub platform: ClientPlatform,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub source: String,

    pub visibility: Visibility,
}
