use crate::command_parser::{CommandsCollection, FolioCommand};

pub fn get_all_commands_collections() -> Vec<CommandsCollection> {
  vec![get_remote_commands_collection(), get_project_commands_collection()]
}

fn get_remote_commands_collection() -> CommandsCollection {
  CommandsCollection::new(
      "remote".to_string(),
      "Manage your remote portfolio projects repositories".to_string(),
      vec![
          FolioCommand::new(
              "list".to_string(),
              "Lists your remote repositories".to_string(),
              |_| true,
          ),
          FolioCommand::new(
              "help".to_string(),
              "Displays available commands".to_string(),
              |_| true,
          ),
      ],
  )
}

fn get_project_commands_collection() -> CommandsCollection {
  CommandsCollection::new(
      "project".to_string(),
      "Manage your portfolio projects".to_string(),
      vec![
          FolioCommand::new(
              "list".to_string(),
              "Lists the projects of the default or specified remote repository".to_string(),
              |_| true,
          ),
          FolioCommand::new(
              "help".to_string(),
              "Displays available commands".to_string(),
              |_| true,
          ),
      ],
  )
}