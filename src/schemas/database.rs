use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use mongodb::{
    results::InsertOneResult,
    sync::{Client as MongoClient, Collection},
};

/// Placeholder type
/// Should be removed before going to production
type Whatever = String;

#[derive(Serialize, Deserialize, Debug)]
pub enum Visibility {
    /**
        If content reported was found to break rules, we set it to ForcedPrivate.
        The user cannot change it from any other status, and will appear as though it was deleted.

        We keep it in the database for moderation & legal purposes, but it will NOT be visible to the public.
    */
    ForcedPrivate,
    Private,
    Unlisted,
    Public,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ThemeClient {
    Android,
    Revite,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientPlatform {
    Android,
    Ios,
    Web,
    Desktop,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReportStatus {
    Pending,
    ActionDealt,
    Ignored,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReportType {
    Bot,
    Server,
    Plugin,
    Client,
    Theme,
}

pub struct Database {
    pub bot: Collection<Bot>,
    pub client: Collection<Client>,
    pub plugin: Collection<Plugin>,
    pub report: Collection<Report>,
    pub server: Collection<Server>,
    pub theme: Collection<Theme>,
}

impl Database {
    pub fn init() -> Self {
        let client =
            MongoClient::with_uri_str("mongodb://revolt.doyouliveinthe.uk:27018/DiscoveryDB")
                .unwrap();
        let db = client.database("DiscoveryDB");

        Database {
            bot: db.collection("bot"),
            client: db.collection("client"),
            plugin: db.collection("plugin"),
            report: db.collection("report"),
            server: db.collection("server"),
            theme: db.collection("theme"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,

    pub developer: String,
    pub platform: ClientPlatform,
    pub source: String,

    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
    #[serde(rename = "_id")]
    pub id: String,

    pub name: String,
    pub description: Option<String>,

    pub avatar: Option<String>,
    pub banner: Option<String>,

    pub developer: String,

    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,
    pub description: Option<String>,

    pub author: String,
    pub data: Whatever,
    pub platform: ThemeClient,

    pub visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    #[serde(rename = "_id")]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub reporter_id: String,
    pub reported_id: String,

    pub reported_type: ReportType,

    pub reason: String,
    pub status: ReportStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugin {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,
    pub description: Option<String>,

    pub developer: String,
    pub source: String,

    pub visibility: Visibility,
}
