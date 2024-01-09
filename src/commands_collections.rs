use crate::command_parser::{CommandsCollection, FolioCommand};

#[derive(Clone)]
pub enum FOLIO_COMMAND_ID {
    REMOTE_LIST,
    REMOTE_HELP,
    PROJECT_LIST,
    PROJECT_HELP,
}

pub fn get_all_commands_collections() -> Vec<CommandsCollection> {
    vec![
        get_remote_commands_collection(),
        get_project_commands_collection(),
    ]
}

fn get_remote_commands_collection() -> CommandsCollection {
    let collection_name = "remote".to_string();
    CommandsCollection::new(
        collection_name.clone(),
        "Manage your remote portfolio projects repositories".to_string(),
        vec![
            FolioCommand::new(
                FOLIO_COMMAND_ID::REMOTE_LIST,
                "list".to_string(),
                "Lists your remote repositories".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FOLIO_COMMAND_ID::REMOTE_HELP,
                "help".to_string(),
                "Displays available commands".to_string(),
                |_| true,
            ),
        ],
    )
}

fn get_project_commands_collection() -> CommandsCollection {
    let collection_name = "project".to_string();
    CommandsCollection::new(
        collection_name.clone(),
        "Manage your portfolio projects".to_string(),
        vec![
            FolioCommand::new(
                FOLIO_COMMAND_ID::PROJECT_LIST,
                "list".to_string(),
                "Lists the projects of the default or specified remote repository".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FOLIO_COMMAND_ID::PROJECT_HELP,
                "help".to_string(),
                "Displays available commands".to_string(),
                |_| true,
            ),
        ],
    )
}
