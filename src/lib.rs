use std::error::Error;
use crate::command::CommandInput;
use crate::context::Context;
use crate::file_manager::LinuxFileManager;

mod file_manager;
mod git_manager;
mod context;
mod context_handler;
#[cfg(test)]
mod utils;
mod scopes;
mod table_builders;
mod command;

pub fn run() -> Result<(), Box<dyn Error>> {
    let command_input = CommandInput::build();
    let scope_table = table_builders::build_scope_table();
    let name_table = table_builders::build_command_name_table();
    // let context = Context::new(Box::new(LinuxFileManager));

    // let _ = context_handler::handle_context(context)?;

    Ok(())
}
