use serde::{Deserialize, Serialize};

use super::{Visibility, Whatever};

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[repr(u8)]
pub enum ThemeClient {
    Android,
    Revite,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Theme {
    name: String,
    description: Option<String>,
    author: String,
    data: Whatever,
    platform: ThemeClient,

    pub visibility: Visibility,
}
