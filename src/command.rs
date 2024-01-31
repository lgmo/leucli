use std::collections::HashMap;
use std::env;

pub struct CommandInput {
    pub name: String,
    pub args: Vec<String>,
}


#[derive(Clone, Debug)]
pub struct Command {
    pub name: String,
    pub execution_list: Vec<String>,
}

impl CommandInput {
    pub fn build(command_table: &HashMap<String, Command>) -> Result<CommandInput, String> {
        let input_args: Vec<String> = env::args().collect();
        if input_args.len() < 2 {
            return Err(String::from("No command provided"));
        }

        let input_name = input_args[1].clone();
        assert_eq!(input_name, "ls".to_string());
        let name;
        match command_table.get(&input_name) {
            Some(command) => { name = command.name.to_string(); },
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