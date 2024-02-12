use crate::command_controller::gh_fetchers::gh_remote_fetcher::get_remotes;
use crate::command_parser::input_parser::is_json_display;
use crate::command_parser::CommandsCollection;
use crate::content_displayer::table_displayer::build_table;
use crate::error::{folio_error::FolioError, parse_command_error::HelpDisplay};

pub fn display_remote_help(commands_collection: &CommandsCollection) -> Result<(), FolioError> {
    println!("{}", commands_collection.get_help());
    Ok(())
}

pub async fn list_remotes(
    gh_auth_token: &String,
    input_args: &Vec<String>,
) -> Result<(), FolioError> {
    let remotes = get_remotes(gh_auth_token).await?;
    if is_json_display(input_args) {
        println!("{}", serde_json::to_string(&remotes)?);
    } else {
        println!("{}", build_table(&remotes)?);
    }
    Ok(())
}
