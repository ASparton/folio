use crate::command_controller::gh_fetchers::gh_remote_fetcher::get_remotes;
use crate::command_displayer::build_table;
use crate::command_parser::CommandsCollection;
use crate::error::{folio_error::FolioError, parse_command_error::HelpDisplay};

pub fn display_remote_help(commands_collection: &CommandsCollection) -> Result<(), FolioError> {
    println!("{}", commands_collection.get_help());
    Ok(())
}

pub async fn list_remotes(gh_auth_token: &String) -> Result<(), FolioError> {
    let remotes = get_remotes(gh_auth_token).await?;
    println!("{}", build_table(&remotes)?);
    Ok(())
}
