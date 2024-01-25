use crate::command_parser::{CommandsCollection, FolioCommand};

#[derive(Clone)]
pub enum FolioCommandId {
    RemoteList,
    RemoteHelp,
    ProjectList,
    ProjectView,
    ProjectHelp,
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
                FolioCommandId::RemoteList,
                "list".to_string(),
                "Lists your remote repositories".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FolioCommandId::RemoteHelp,
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
                FolioCommandId::ProjectList,
                "list".to_string(),
                "Lists the projects of the default or specified remote repository".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FolioCommandId::ProjectView,
                "view".to_string(),
                "View one project of the default or specified remote repository".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FolioCommandId::ProjectHelp,
                "help".to_string(),
                "Displays available commands".to_string(),
                |_| true,
            ),
        ],
    )
}
