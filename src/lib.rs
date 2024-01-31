use std::error::Error;
use crate::command::CommandInput;
pub use simple_home_dir;
use crate::config::Config;

mod file_manager;
mod git_manager;
mod command_input_handler;
mod scopes;
mod table_builders;
mod command;
#[cfg(test)]
mod utils;
mod config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::build()?;
    let scope_table = table_builders::build_scope_table(&config.scopes)?;
    let (command_table, scope_table) = table_builders::build_command_name_table(scope_table, &config.commands)?;
    let command_input = CommandInput::build(&command_table)?;

    command_input_handler::handle(command_input, scope_table, command_table)?;
    // let context = Context::new(Box::new(LinuxFileManager));

    // let _ = command_input_handler::handle_context(context)?;

    Ok(())
}
