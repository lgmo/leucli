use std::error::Error;
use crate::command::CommandInput;

mod file_manager;
mod git_manager;
mod command_input_handler;
mod scopes;
mod table_builders;
mod command;
#[cfg(test)]
mod utils;

pub fn run() -> Result<(), Box<dyn Error>> {

    let scope_table = table_builders::build_scope_table();
    let name_table = table_builders::build_command_name_table();
    let command_input = CommandInput::build(name_table)?;

    command_input_handler::handle(command_input, scope_table)?;
    // let context = Context::new(Box::new(LinuxFileManager));

    // let _ = command_input_handler::handle_context(context)?;

    Ok(())
}
