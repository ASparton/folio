use command_parser::parse_command;
use commands_controller::execute_command;
use error::folio_error::FolioError;
use gh_token_getter::get_gh_token;
use input_getter::get_input_args;

mod command_parser;
mod commands_collections;
mod commands_controller;
mod error;
mod gh_fetchers;
mod gh_reqwestor;
mod gh_token_getter;
mod input_getter;

#[tokio::main]
async fn main() -> Result<(), FolioError> {
	
	let input_args = get_input_args();
    let gh_auth_token = get_gh_token(&input_args)?;
    let (commands_collection, command) = parse_command(&input_args)?;
    execute_command(
        &command.get_id(),
        &commands_collection,
        &gh_auth_token,
        &input_args,
    )
    .await
}
