use std::fmt::Display;

use crate::command_parser::{CommandsCollection, FolioCommand};

/// Raised when an error occured when parsing a string into a command.
pub struct ParseCommandError {
    attempted_command: String,
    commands_collection: Option<CommandsCollection>,
    command: Option<FolioCommand>,
}

/// Used to display the help the user will see if he uses a command wrongly.
pub trait HelpDisplay {
    fn get_help(&self) -> String;
    fn get_help_and_usage(&self) -> String;
}

impl ParseCommandError {
    pub fn new(
        attempted_command: String,
        commands_collection: Option<CommandsCollection>,
        command: Option<FolioCommand>,
    ) -> ParseCommandError {
        ParseCommandError {
            attempted_command,
            commands_collection,
            command,
        }
    }
}

impl Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.commands_collection {
            None => match &self.command {
                None => write!(
                    f,
                    "Unknown command `{}`. Try 'help' to see available commands.",
                    self.attempted_command
                ),
                Some(command) => write!(f, "{}", command.get_help()),
            },
            Some(commands_collection) => match &self.command {
                None => write!(
                    f,
                    "Command `{}` does not exist in collection `{}`.\n{}",
                    self.attempted_command,
                    commands_collection.get_name(),
                    commands_collection.get_help()
                ),
                Some(command) => write!(
                    f,
                    "Invalid arguments for command `{}`.\n{}",
                    commands_collection.get_name() + " " + &command.get_name(),
                    command.get_help_and_usage()
                ),
            },
        }
    }
}
