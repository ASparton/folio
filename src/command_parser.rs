mod command;
mod commands_collection;

pub use command::FolioCommand;
pub use commands_collection::CommandsCollection;

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
pub fn parse_command(input: &String) -> Result<FolioCommand, ParseCommandError> {
    let input_words: Vec<&str> = input.split(' ').collect();

    let targeted_commands_collection = get_targeted_commands_collection(&input_words)?;
    let targeted_command =
        get_targeted_command_in_collection(&targeted_commands_collection, &input_words)?;

    let command_args = &input_words.get(2..).unwrap().to_vec();
    if !targeted_command.verify_args(command_args) {
        return Err(ParseCommandError::new(
            Some(targeted_commands_collection),
            Some(targeted_command),
        ));
    }

    Ok(targeted_command)
}

fn get_targeted_commands_collection(
    input_words: &Vec<&str>,
) -> Result<CommandsCollection, ParseCommandError> {
    if input_words.len() == 0 {
        return Err(ParseCommandError::new(None, None));
    }

    let commands_collections = commands_collections::get_all_commands_collections();
    match commands_collections
        .into_iter()
        .find(|collection| collection.get_name().eq(input_words[0]))
    {
        None => Err(ParseCommandError::new(None, None)),
        Some(collection) => Ok(collection),
    }
}

fn get_targeted_command_in_collection(
    targeted_commands_collection: &CommandsCollection,
    input_words: &Vec<&str>,
) -> Result<FolioCommand, ParseCommandError> {
    if input_words.len() < 2 {
        return Err(ParseCommandError::new(
            Some(targeted_commands_collection.clone()),
            None,
        ));
    }

    match targeted_commands_collection
        .get_commands()
        .into_iter()
        .find(|command| command.get_name().eq(input_words[1]))
    {
        None => Err(ParseCommandError::new(
            Some(targeted_commands_collection.clone()),
            None,
        )),
        Some(command_found) => Ok(command_found),
    }
}
