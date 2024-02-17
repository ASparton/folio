use crate::command_controller::folio_dialoguer::{ask_for_confirmation, make_selection};
use crate::command_controller::gh_fetchers::gh_project_fetcher::{
    delete_project_of_remote, get_project_of_remote, list_projects_name_of_remote,
    list_projects_of_remote,
};
use crate::command_controller::gh_fetchers::gh_remote_fetcher::get_remotes_name;
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
    let selected_remote_name = ask_remote_name(gh_auth_token).await?;
    let projects = list_projects_of_remote(&selected_remote_name, gh_auth_token).await?;
    if is_json_display(input_args) {
        println!("{}", serde_json::to_string(&projects)?);
    } else {
        println!("{}", build_table(&projects)?);
    }
    Ok(())
}

pub async fn view_project(
    gh_auth_token: &String,
    input_args: &Vec<String>,
) -> Result<(), FolioError> {
    let selected_remote_name = ask_remote_name(gh_auth_token).await?;
    let projects_name = list_projects_name_of_remote(&selected_remote_name, gh_auth_token).await?;
    if projects_name.is_empty() {
        println!("No projects found in remote `{}`", selected_remote_name);
        return Ok(());
    }

    let selected_project_index =
        make_selection(&projects_name, "Select the project you want to view:")?;
    let selected_project_name = &projects_name[selected_project_index];
    let project =
        get_project_of_remote(&selected_remote_name, selected_project_name, gh_auth_token).await?;
    if is_json_display(input_args) {
        println!("{}", serde_json::to_string(&project)?);
    } else {
        println!("{}", build_table(&vec![project])?);
    }
    Ok(())
}

pub async fn delete_project(gh_auth_token: &String) -> Result<(), FolioError> {
    let selected_remote_name = ask_remote_name(gh_auth_token).await?;
    let projects_name = list_projects_name_of_remote(&selected_remote_name, gh_auth_token).await?;
    if projects_name.is_empty() {
        println!("No projects found in remote `{}`", selected_remote_name);
        return Ok(());
    }

    let selected_project_index =
        make_selection(&projects_name, "Select the project you want to delete:")?;
    let selected_project_name = &projects_name[selected_project_index];
    let deletion_confirmed = ask_for_confirmation("Are you sure you want to delete this project?")?;
    if !deletion_confirmed {
        println!("Project deletion aborted");
        return Ok(());
    }

    let deletion_successful =
        delete_project_of_remote(&selected_remote_name, selected_project_name, gh_auth_token)
            .await?;
    if deletion_successful {
        println!("Project `{}` deleted successfully", selected_project_name);
    } else {
        return Err(FolioError {
            message: "Project deletion failed".to_string(),
        });
    }
    Ok(())
}

pub fn new_project(gh_auth_token: &String, input_args: &Vec<String>) -> Result<(), FolioError> {
    println!("Given path: {}", input_args[0]);
    Ok(())
}

async fn ask_remote_name(gh_auth_token: &String) -> Result<String, FolioError> {
    let remotes_name = get_remotes_name(gh_auth_token).await?;
    let selected_remote_index = make_selection(&remotes_name, "Select your remote portfolio:")?;
    Ok(remotes_name[selected_remote_index].clone())
}
