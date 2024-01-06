use crate::command_parser::command::FolioCommand;

use crate::error::HelpDisplay;

/// Defines a collection of commands that are related. 
/// They all act on one kind of object.
pub struct CommandsCollection {
    name: String,
    description: String,
    commands: Vec<FolioCommand>,
}

impl CommandsCollection {
    pub fn new(name: String, description: String, commands: Vec<FolioCommand>) -> CommandsCollection {
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

    pub fn clone(&self) -> CommandsCollection {
        CommandsCollection::new(
            self.get_name(),
            self.description.clone(),
            self.get_commands(),
        )
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
}
