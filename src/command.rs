use std::env;

pub struct CommandInput {
    pub name: String,
    pub args: Vec<String>,
}

impl CommandInput {
    pub fn build() -> Result<CommandInput, String> {
        let input_args: Vec<String> = env::args().collect();
        if input_args.len() < 2 {
            return Err(String::from("No command provided"));
        }

        let name = input_args[1].clone();
        let args: Vec<String>;
        if input_args.len() > 2 {
            args = Vec::from(&input_args[2..]);
        } else {
            args = Vec::new();
        }

        Ok( CommandInput { name, args } )
    }
}