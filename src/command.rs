use std::collections::HashMap;
use std::env;
use crate::scopes::Scope;

pub struct CommandInput {
    pub name: String,
    pub args: Vec<String>,
}

pub struct Command {
    pub  name: String,
    pub args: Vec<String>,
    pub scope: Scope,
}

impl CommandInput {
    pub fn build(name_table: HashMap<String, String>) -> Result<CommandInput, String> {
        let input_args: Vec<String> = env::args().collect();
        if input_args.len() < 2 {
            return Err(String::from("No command provided"));
        }

        let input_name = input_args[1].clone();
        let name;
        match name_table.get(&input_name) {
            Some(command_name) => { name = command_name.to_owned(); },
            _ => {
                return Err(format!("There is no command with name or alias '{}'", input_name)
                    .to_string());
            },
        }

        let args: Vec<String>;
        if input_args.len() > 2 {
            args = Vec::from(&input_args[2..]);
        } else {
            args = Vec::new();
        }

        Ok( CommandInput { name, args } )
    }
}