use std::fmt::{Debug, Display};

use crate::error::{
    gh_reqwestor_error::GhReqwestError, missing_gh_token_error::MissingGHTokenError,
    parse_command_error::ParseCommandError,
};

/// Raised when an error occured when parsing a string into a command.
#[derive(Clone)]
pub struct FolioError {
    pub message: String,
}

impl FolioError {
    pub fn empty_new() -> FolioError {
        FolioError {
            message: "".to_string()
        }
    }
}

impl Debug for FolioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for FolioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<GhReqwestError> for FolioError {
    fn from(value: GhReqwestError) -> Self {
        FolioError {
            message: format!("{}", value),
        }
    }
}

impl From<MissingGHTokenError> for FolioError {
    fn from(value: MissingGHTokenError) -> Self {
        FolioError {
            message: format!("{}", value),
        }
    }
}

impl From<ParseCommandError> for FolioError {
    fn from(value: ParseCommandError) -> Self {
        FolioError {
            message: format!("{}", value),
        }
    }
}

impl From<dialoguer::Error> for FolioError {
    fn from(_value: dialoguer::Error) -> Self {
        FolioError::empty_new()
    }
}

impl From<serde_json::Error> for FolioError {
    fn from(value: serde_json::Error) -> Self {
        FolioError {
            message: format!("{}", value),
        }
    }
}
