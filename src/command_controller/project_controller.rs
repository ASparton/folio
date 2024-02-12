use crate::command_controller::folio_dialoguer::make_selection;
use crate::command_controller::gh_fetchers::{
    gh_project_fetcher::list_projects_of_remote, gh_remote_fetcher::get_remotes_name,
};
use crate::command_parser::input_parser::is_json_display;
use crate::command_parser::CommandsCollection;
use crate::content_displayer::table_displayer::build_table;
use crate::error::folio_error::FolioError;
use crate::error::parse_command_error::HelpDisplay;

pub fn display_project_help(commands_collection: &CommandsCollection) -> Result<(), FolioError> {
    println!("{}", commands_collection.get_help());
    Ok(())
}

pub async fn list_projects(
    gh_auth_token: &String,
    input_args: &Vec<String>,
) -> Result<(), FolioError> {
    let remotes_name = get_remotes_name(gh_auth_token).await?;
    let selected_remote_index = make_selection(&remotes_name, "Select your remote portfolio: ")?;
    let selected_remote_name = &remotes_name[selected_remote_index];
    let projects = list_projects_of_remote(selected_remote_name, gh_auth_token).await?;
    if is_json_display(input_args) {
        println!("{}", serde_json::to_string(&projects)?);
    } else {
        println!("{}", build_table(&projects)?);
    }
    Ok(())
}

pub fn view_project() -> Result<(), FolioError> {
    Err(FolioError {
        message: "Not implemented".to_string(),
    })
}
