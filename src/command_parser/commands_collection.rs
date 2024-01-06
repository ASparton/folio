use crate::command_parser::command::Command;

use crate::error::HelpDisplay;

/// Defines a collection of commands that are related. 
/// They all act on one kind of object.
pub struct CommandsCollection {
    name: String,
    description: String,
    commands: Vec<Command>,
}

impl CommandsCollection {
    pub fn new(name: String, description: String, commands: Vec<Command>) -> CommandsCollection {
        CommandsCollection {
            name,
            description,
            commands,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_commands(&self) -> Vec<Command> {
        let mut commands: Vec<Command> = Vec::new();
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
            help_message.push_str(format!("  {}", command.get_help()).as_str());
        }
        help_message
    }
}
