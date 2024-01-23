use crate::command_parser::CommandsCollection;
use crate::commands_collections::FOLIO_COMMAND_ID;
use crate::error::folio_error::FolioError;

mod project_controller;
mod remote_controller;

pub async fn execute_command(
    command_id: &FOLIO_COMMAND_ID,
    commands_collection: &CommandsCollection,
    gh_auth_token: &String,
    _args: &Vec<String>,
) -> Result<(), FolioError> {
    match command_id {
        FOLIO_COMMAND_ID::RemoteHelp => remote_controller::display_remote_help(commands_collection),
        FOLIO_COMMAND_ID::RemoteList => remote_controller::list_remotes(gh_auth_token).await,
        FOLIO_COMMAND_ID::ProjectHelp => {
            project_controller::display_project_help(commands_collection)
        }
        FOLIO_COMMAND_ID::ProjectList => project_controller::list_projects(),
        FOLIO_COMMAND_ID::ProjectView => project_controller::view_project(),
    }
}
