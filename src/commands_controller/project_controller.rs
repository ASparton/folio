use dialoguer::theme::ColorfulTheme;
use dialoguer::FuzzySelect;
use tabled::settings::{Style, Width, measurement::Percent};
use tabled::Table;

use crate::command_parser::CommandsCollection;
use crate::error::folio_error::FolioError;
use crate::error::parse_command_error::HelpDisplay;
use crate::gh_fetchers::gh_project_fetcher::list_projects_of_remote;
use crate::gh_fetchers::gh_remote_fetcher::get_remotes_name;

pub fn display_project_help(commands_collection: &CommandsCollection) -> Result<(), FolioError> {
    println!("{}", commands_collection.get_help());
    Ok(())
}

pub async fn list_projects(gh_auth_token: &String) -> Result<(), FolioError> {
    let remotes_name = get_remotes_name(gh_auth_token).await?;
    let selected_remote_index = match FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("\nWhich portfolio do you want to see?")
        .default(0)
        .items(&remotes_name)
        .interact_opt()?
    {
        None => return Err(FolioError::empty_new()),
        Some(index) => index,
    };

    let projects =
        list_projects_of_remote(&remotes_name[selected_remote_index], gh_auth_token).await?;
    println!(
        "{}",
        Table::new(&projects)
            .with(Style::rounded())
            .with(Width::wrap(100).keep_words())
    );
    Ok(())
}

pub fn view_project() -> Result<(), FolioError> {
    Err(FolioError {
        message: "Not implemented".to_string(),
    })
}
