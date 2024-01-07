use std::fmt::Display;

use crate::command_parser::{CommandsCollection, FolioCommand};

/// Raised when an error occured when parsing a string into a command.
pub struct ParseCommandError {
    commands_collection: Option<CommandsCollection>,
    command: Option<FolioCommand>,
}

/// Used to display the help the user will see if he uses a command wrongly.
pub trait HelpDisplay {
    fn get_help(&self) -> String;
}

impl ParseCommandError {
    pub fn new(
        commands_collection: Option<CommandsCollection>,
        command: Option<FolioCommand>,
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
                    "Unkown `{}` command\n{}",
                    commands_collection.get_name(),
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
