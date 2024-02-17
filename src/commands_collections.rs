use crate::command_parser::{
    input_parser::str_is_valid_path_to_folder, CommandsCollection, FolioCommand,
};

#[derive(Clone)]
pub enum FolioCommandId {
    RemoteList,
    RemoteHelp,

    ProjectList,
    ProjectView,
    ProjectDelete,
    ProjectNew,
    ProjectHelp,
}

pub fn get_all_commands_collections() -> Vec<CommandsCollection> {
    vec![
        get_remote_commands_collection(),
        get_project_commands_collection(),
    ]
}

fn get_remote_commands_collection() -> CommandsCollection {
    CommandsCollection::new(
        "remote".to_string(),
        "Manage your remote portfolio projects repositories".to_string(),
        vec![
            FolioCommand::new(
                FolioCommandId::RemoteList,
                "list".to_string(),
                "Lists your remote repositories".to_string(),
                "remote list".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FolioCommandId::RemoteHelp,
                "help".to_string(),
                "Displays available commands".to_string(),
                "remote help".to_string(),
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
                FolioCommandId::ProjectList,
                "list".to_string(),
                "Lists the projects of the default or chosen remote repository".to_string(),
                "project list".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FolioCommandId::ProjectView,
                "view".to_string(),
                "View one project of the default or chosen remote repository".to_string(),
                "project view".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FolioCommandId::ProjectDelete,
                "delete".to_string(),
                "Delete one project of the default or chosen remote repository".to_string(),
                "project delete".to_string(),
                |_| true,
            ),
            FolioCommand::new(
                FolioCommandId::ProjectNew,
                "new".to_string(),
                "Create a new project inside the default or chosen remote repository".to_string(),
                "project new [PATH]".to_string(),
                |args: &Vec<String>| -> bool {
                    args.len() >= 1 && str_is_valid_path_to_folder(&args[0])
                },
            ),
            FolioCommand::new(
                FolioCommandId::ProjectHelp,
                "help".to_string(),
                "Displays available commands".to_string(),
                "project help".to_string(),
                |_| true,
            ),
        ],
    )
}
