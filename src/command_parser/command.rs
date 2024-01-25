use crate::{commands_collections::FolioCommandId, error::parse_command_error::HelpDisplay};

/// Identify a folio CLI command.
#[derive(Clone)]
pub struct FolioCommand {
    id: FolioCommandId,
    name: String,
    description: String,
    verify_args_func: fn(&Vec<String>) -> bool,
}

impl FolioCommand {
    pub fn new(
        id: FolioCommandId,
        name: String,
        description: String,
        verify_args_func: fn(&Vec<String>) -> bool,
    ) -> FolioCommand {
        FolioCommand {
            id,
            name,
            description,
            verify_args_func,
        }
    }

    pub fn get_id(&self) -> FolioCommandId {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Verify if the given arguments are valid for this command.
    pub fn verify_args(&self, args: &Vec<String>) -> bool {
        (self.verify_args_func)(args)
    }
}

impl HelpDisplay for FolioCommand {
    fn get_help(&self) -> String {
        format!("{}		{}", self.name, self.description)
    }
}
