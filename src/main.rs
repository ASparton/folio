use controllers::project_controller::get_project_of_remote;

mod command_parser;
mod commands_collections;
mod controllers;
mod error;
mod gh_reqwestor;
mod gh_token_getter;
mod input_getter;

#[tokio::main]
async fn main() {
    match get_project_of_remote(
        &"asparton-portfolio".to_string(),
        &"OpenFlappy".to_string(),
        &"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",
    )
    .await
    {
        Err(_error) => println!("Invalid remote or project name."),
        Ok(project) => println!("{:?}", project),
    }
}
