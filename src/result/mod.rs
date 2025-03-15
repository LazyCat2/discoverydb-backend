pub mod result;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

// Code borrowed from https://github.com/revoltchat/backend/tree/main/crates/core/result/src
// Thank you Revolt!

/// Result type with custom Error
// pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    /// Type of error and additional information
    #[serde(flatten)]
    pub error_type: ErrorType,

    /// Where this error occurred
    pub location: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} occurred in {}", self.error_type, self.location)
    }
}

impl std::error::Error for Error {}

/// Possible error types
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ErrorType {
    /// This error was not labeled :(
    LabelMe,

    // ? User related errors
    UsernameTaken,
    InvalidUsername,

    // ? Revolt related errors
    UnknownServer,
    UnknownBot,

    // ? Reporting errors
    CannotReportYourself,

    // ? Account errors
    NoAccess,
    /// No access to the data (e.g. trying to access a listing that was privated)
    NotAllowed,
    /// Not allowed to perform the action (e.g. trying to delete a listing you don't own)
    InvalidCredentials,
    InvalidSession,
    NotAuthenticated,

    // ? General errors
    DatabaseError {
        error: String,
    },
    InternalError,
    InvalidOperation,
    InvalidProperty,
    NotFound,
    NoEffect,
    FailedValidation {
        error: String,
    },
}

#[macro_export]
macro_rules! create_error {
    ( $error: ident $( $tt:tt )? ) => {
        $crate::result::Error {
            error_type: $crate::result::ErrorType::$error $( $tt )?,
            location: format!("{}:{}:{}", file!(), line!(), column!()),
        }
    };
}
