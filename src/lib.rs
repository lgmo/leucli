use std::error::Error;
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

pub fn run() -> Result<(), Box<dyn Error>> {
    // let context = Context::new(Box::new(LinuxFileManager));

    // let _ = context_handler::handle_context(context)?;

    Ok(())
}
