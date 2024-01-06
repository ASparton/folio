use command_parser::parse_command;
use error::HelpDisplay;

mod command_parser;
mod commands_collections;
mod controllers;
mod error;
mod gh_reqwestor;

fn main() {
    let command = parse_command(&"project jk".to_string());
    match command {
        Err(error) => println!("{}", error),
        Ok(c) => println!("{}", c.get_help()),
    }
}
