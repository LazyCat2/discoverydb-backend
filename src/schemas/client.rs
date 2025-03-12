use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[repr(u8)]
pub enum ClientPlatform {
    Android,
    Ios,
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
}
