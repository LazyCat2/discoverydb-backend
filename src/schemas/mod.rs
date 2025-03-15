mod bot;
mod client;
mod plugin;
mod server;
mod theme;
mod report;

/// Placeholder type
/// Should be removed before going to production
type Whatever = String;

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[repr(u8)]
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

use rocket::serde::{Deserialize, Serialize};
pub use bot::*;
pub use client::*;
pub use plugin::*;
pub use server::*;
pub use theme::*;
