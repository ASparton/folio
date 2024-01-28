mod command_controller;
mod command_displayer;
mod command_parser;
mod commands_collections;
mod error;

use crate::command_controller::execute_command;
use crate::command_parser::gh_token_getter::get_gh_token;
use crate::command_parser::input_getter::get_input_args;
use crate::command_parser::parse_command;
use crate::error::folio_error::FolioError;

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
