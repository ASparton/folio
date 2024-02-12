mod folio_dialoguer;
mod project_controller;
mod remote_controller;
mod gh_fetchers;

use crate::command_parser::CommandsCollection;
use crate::commands_collections::FolioCommandId;
use crate::error::folio_error::FolioError;

pub async fn execute_command(
    command_id: &FolioCommandId,
    commands_collection: &CommandsCollection,
    gh_auth_token: &String,
    _args: &Vec<String>,
) -> Result<(), FolioError> {
    match command_id {
        FolioCommandId::RemoteHelp => remote_controller::display_remote_help(commands_collection),
        FolioCommandId::RemoteList => remote_controller::list_remotes(gh_auth_token).await,
        FolioCommandId::ProjectHelp => {
            project_controller::display_project_help(commands_collection)
        }
        FolioCommandId::ProjectList => project_controller::list_projects(gh_auth_token).await,
        FolioCommandId::ProjectView => project_controller::view_project(),
    }
}
