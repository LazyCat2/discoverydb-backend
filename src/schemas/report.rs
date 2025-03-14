use rocket::serde::{Deserialize, Serialize};
use crate::schemas::Visibility;

/*
    "id"	INTEGER NOT NULL UNIQUE,
    "reporter_id"	TEXT NOT NULL,
    "reported_id"	TEXT NOT NULL,
    "reported_type" INTEGER NOT NULL,
    "reason"	TEXT NOT NULL,
    "status"	INTEGER NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
*/

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[repr(u8)]
pub enum ReportStatus {
    Pending,
    ActionDealt,
    Ignored,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[repr(u8)]
pub enum ReportType {
    Bot,
    Server,
    Plugin,
    Client,
    Theme
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Report {
    pub id: u16,

    pub reporter_id: String,
    pub reported_id: String,

    pub reported_type: ReportType,

    pub reason: String,
    pub status: ReportStatus,
}
