use std::error::Error;
use crate::command::CommandInput;
use crate::context::Context;
use crate::file_manager::LinuxFileManager;

mod file_manager;
mod git_manager;
mod context;
mod command_input_handler;
mod scopes;
mod table_builders;
mod command;

pub fn run() -> Result<(), String> {

    let scope_table = table_builders::build_scope_table();
    let name_table = table_builders::build_command_name_table();
    let command_input ;
    match CommandInput::build(name_table) {
        Ok(ci) => { command_input = ci; },
        Err(e) => { return Err(e.to_string()); },
    };

    command_input_handler::handle(command_input, scope_table);
    // let context = Context::new(Box::new(LinuxFileManager));

    // let _ = command_input_handler::handle_context(context)?;

    Ok(())
}
