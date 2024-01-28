mod command;
mod commands_collection;
pub mod gh_token_getter;
pub mod input_getter;

pub use crate::command_parser::{command::FolioCommand, commands_collection::CommandsCollection};
use crate::commands_collections;
use crate::error::parse_command_error::ParseCommandError;

/// Parse the given input into a folio command.
///
/// # Examples
///
/// ```
/// let input = "remote list".to_string();
/// let command = parse_command(&input);
/// assert!(command.is_some());
///
/// let input = "wrong command".to_string();
/// let command = parse_command(&input);
/// assert!(command.is_err());
/// ```
pub fn parse_command(
    input_args: &Vec<String>,
) -> Result<(CommandsCollection, FolioCommand), ParseCommandError> {
    let targeted_commands_collection = get_targeted_commands_collection(input_args)?;
    let targeted_command =
        get_targeted_command_in_collection(&targeted_commands_collection, input_args)?;

    let command_args = &input_args.get(2..).unwrap().to_vec();
    if !targeted_command.verify_args(command_args) {
        return Err(ParseCommandError::new(
            Some(targeted_commands_collection),
            Some(targeted_command),
        ));
    }

    Ok((targeted_commands_collection, targeted_command))
}

fn get_targeted_commands_collection(
    input_args: &Vec<String>,
) -> Result<CommandsCollection, ParseCommandError> {
    if input_args.len() == 0 {
        return Err(ParseCommandError::new(None, None));
    }

    let commands_collections = commands_collections::get_all_commands_collections();
    match commands_collections
        .into_iter()
        .find(|collection| collection.get_name().eq(&input_args[0]))
    {
        None => Err(ParseCommandError::new(None, None)),
        Some(collection) => Ok(collection),
    }
}

fn get_targeted_command_in_collection(
    targeted_commands_collection: &CommandsCollection,
    input_args: &Vec<String>,
) -> Result<FolioCommand, ParseCommandError> {
    if input_args.len() < 2 {
        return Err(ParseCommandError::new(
            Some(targeted_commands_collection.clone()),
            None,
        ));
    }

    match targeted_commands_collection
        .get_commands()
        .into_iter()
        .find(|command| command.get_name().eq(&input_args[1]))
    {
        None => Err(ParseCommandError::new(
            Some(targeted_commands_collection.clone()),
            None,
        )),
        Some(command_found) => Ok(command_found),
    }
}
