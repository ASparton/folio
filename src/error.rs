use std::{collections::HashMap, fmt::Display};

use reqwest::StatusCode;

use crate::command_parser::{Command, CommandsCollection};

/// Raised when an error occured during a request to the Github API.
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

/// Raised when an error occured when parsing a string into a command.
pub struct ParseCommandError {
    commands_collection: Option<CommandsCollection>,
    command: Option<Command>,
}

/// Used to display the help the user will see if he uses a command wrongly.
pub trait HelpDisplay {
    fn get_help(&self) -> String;
}

impl ParseCommandError {
    pub fn new(
        commands_collection: Option<CommandsCollection>,
        command: Option<Command>,
    ) -> ParseCommandError {
        ParseCommandError {
            commands_collection,
            command,
        }
    }
}

impl Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.commands_collection {
            None => match &self.command {
                None => write!(f, "Unknown command. Try 'help' to see available commands."),
                Some(command) => write!(f, "{}", command.get_help()),
            },
            Some(commands_collection) => match &self.command {
                None => write!(
                    f,
                    "Unkown `remote` command.\n{}",
                    commands_collection.get_help()
                ),
                Some(command) => write!(
                    f,
                    "{}\n{}",
                    commands_collection.get_help(),
                    command.get_help()
                ),
            },
        }
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
