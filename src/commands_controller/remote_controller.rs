use crate::command_parser::CommandsCollection;
use crate::error::folio_error::FolioError;
use crate::error::parse_command_error::HelpDisplay;
use crate::gh_fetchers::gh_remote_fetcher::get_remotes;

pub fn display_remote_help(commands_collection: &CommandsCollection) -> Result<(), FolioError> {
    println!("{}", commands_collection.get_help());
    Ok(())
}

pub async fn list_remotes(gh_auth_token: &String) -> Result<(), FolioError> {
    let remotes = get_remotes(gh_auth_token).await?;
    let mut remotes_str: String = String::new();
    for remote in remotes {
        remotes_str += remote.to_string().as_str();
    }
    println!("{}", remotes_str);
    Ok(())
}
