use serde::{Deserialize, Serialize};

use super::{ClientPlatform, ThemeClient};

/// Placeholder type
/// Should be removed before going to production
type Whatever = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
pub enum Visibility {
    /**
        If content was found to break rules, we set it to Locked.
        This visibility status acts the same as Private, accept the submitter cannot change the visibility.

        TODO: If moderator chat is implemented, we can also send a message to the submitter to let them know.
    */
    Locked = -1,

    /**
        Only the submitter (and moderators) can see the content.
        Hidden from:
        - Search
        - Public listings
        - Direct links
    */
    Private = 0,

    /**
        Only the submitter (and moderators), and anyone with a direct link, can see the content.
        Hidden from:
        - Search
        - Public listings

        Seen in:
        - Direct links
    */
    Unlisted = 1,

    /**
        The content is visible to everyone.
        Seen in:
        - Search
        - Public listings
        - Direct links
    */
    Public = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Listing {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub name: String,
    pub description: Option<String>,

    pub visibility: Visibility,
    pub tags: Vec<String>,
    // pub created: Timestamp,
    // pub updated: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    #[serde(flatten)]
    pub listing: Listing,

    pub icon: Option<String>,
    pub banner: Option<String>,

    pub owner: String,
    pub invite: String,

    pub members: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bot {
    #[serde(flatten)]
    pub listing: Listing,

    pub avatar: Option<String>,
    pub banner: Option<String>,

    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    #[serde(flatten)]
    pub listing: Listing,
    pub slug: String,

    pub authors: Vec<String>,

    pub data: Whatever,
    pub platform: ThemeClient,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugin {
    #[serde(flatten)]
    pub listing: Listing,

    pub source: String,
    pub instructions: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    #[serde(flatten)]
    pub listing: Listing,

    pub icon: Option<String>,

    pub source: String,
    pub instructions: String,

    pub platform: ClientPlatform,
}
