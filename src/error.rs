use std::{collections::HashMap, fmt::Display};

use reqwest::StatusCode;

#[derive(Debug)]
pub struct GhReqwestError {
    code: StatusCode,
    message: String,
}

impl From<reqwest::Error> for GhReqwestError {
    fn from(error: reqwest::Error) -> Self {
        println!("{}", error);
        GhReqwestError {
            code: match error.status() {
                None => StatusCode::FORBIDDEN,
                Some(status) => status,
            },
            message: match error.status() {
                None => "No GitHub authentication token provided.".to_string(),
                Some(status) => get_message_from_satus_code(&status),
            },
        }
    }
}

impl Display for GhReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code: {}\nMessage: {}", self.code, self.message)
    }
}

/// Get the error message associated with the given HTTP status code.
///
/// # Example
///
/// ```
/// let status = StatusCode::FORBIDDEN;
/// let message = get_message_from_satus_code(&status);
/// assert_eq!("No GitHub authentication token provided.".to_string(), message);
/// ```
fn get_message_from_satus_code(status_code: &StatusCode) -> String {
    let status_code_to_message: HashMap<StatusCode, String> = HashMap::from([(
        StatusCode::FORBIDDEN,
        "No GitHub authentication token provided.".to_string(),
    )]);

    match status_code_to_message.get(&status_code) {
        None => "Unkown error".to_string(),
        Some(message) => message.clone(),
    }
}
