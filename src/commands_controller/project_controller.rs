use crate::command_parser::CommandsCollection;
use crate::error::folio_error::FolioError;
use crate::error::parse_command_error::HelpDisplay;

pub fn display_project_help(commands_collection: &CommandsCollection) -> Result<(), FolioError> {
    println!("{}", commands_collection.get_help());
    Ok(())
}

pub fn list_projects() -> Result<(), FolioError> {
    Err(FolioError {
        message: "Not implemented".to_string(),
    })
}

pub fn view_project() -> Result<(), FolioError> {
    Err(FolioError {
        message: "Not implemented".to_string(),
    })
}
