use std::io::Cursor;

use crate::result::{Error, ErrorType};
use rocket::serde::json::serde_json;
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};

/// HTTP response builder for Error enum
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = match self.error_type {
            ErrorType::LabelMe => Status::InternalServerError,

            ErrorType::InvalidUsername => Status::BadRequest,
            ErrorType::UsernameTaken => Status::Conflict,

            ErrorType::UnknownServer => Status::NotFound,
            ErrorType::UnknownBot => Status::NotFound,

            ErrorType::CannotReportYourself => Status::BadRequest,

            ErrorType::NoAccess => Status::Forbidden,
            ErrorType::NotAllowed => Status::Forbidden,

            ErrorType::DatabaseError { .. } => Status::InternalServerError,
            ErrorType::InternalError => Status::InternalServerError,
            ErrorType::InvalidOperation => Status::BadRequest,
            ErrorType::InvalidCredentials => Status::Unauthorized,
            ErrorType::InvalidProperty => Status::BadRequest,
            ErrorType::InvalidSession => Status::Unauthorized,
            ErrorType::NotAuthenticated => Status::Unauthorized,
            ErrorType::NotFound => Status::NotFound,
            ErrorType::NoEffect => Status::Ok,
            ErrorType::FailedValidation { .. } => Status::BadRequest,

        };

        // Serialize the error data structure into JSON.
        let string = serde_json::to_string(&self).unwrap();

        // Build and send the request.
        Response::build()
            .sized_body(string.len(), Cursor::new(string))
            .header(ContentType::new("application", "json"))
            .status(status)
            .ok()
    }
}