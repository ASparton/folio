use crate::command_parser::command::FolioCommand;

use crate::error::parse_command_error::HelpDisplay;

/// Defines a collection of commands that are related.
/// They all act on one kind of object.
#[derive(Clone)]
pub struct CommandsCollection {
    name: String,
    description: String,
    commands: Vec<FolioCommand>,
}

impl CommandsCollection {
    pub fn new(
        name: String,
        description: String,
        commands: Vec<FolioCommand>,
    ) -> CommandsCollection {
        CommandsCollection {
            name,
            description,
            commands,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_commands(&self) -> Vec<FolioCommand> {
        let mut commands: Vec<FolioCommand> = Vec::new();
        for command in self.commands.iter() {
            commands.push(command.clone())
        }
        commands
    }
}

impl HelpDisplay for CommandsCollection {
    fn get_help(&self) -> String {
        let mut help_message: String = format!("{}:\n  {}\n", self.name, self.description);
        help_message.push_str("\nCommands:\n");
        for command in self.commands.iter() {
            help_message.push_str(format!("  {}\n", command.get_help()).as_str());
        }
        help_message
    }

    fn get_help_and_usage(&self) -> String {
        let mut help_message: String = format!("{}:\n  {}\n", self.name, self.description);
        help_message.push_str("\nCommands:\n");
        for command in self.commands.iter() {
            help_message.push_str(format!("  {}\n", command.get_help_and_usage()).as_str());
        }
        help_message
    }
}
