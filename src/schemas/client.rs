use serde::{Deserialize, Serialize};

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
    pub platform: u8,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub source: String,
}
