use std::error::Error;
use crate::context::Context;

mod file_manager;
mod git_manager;
mod context;
mod context_handler;
#[cfg(test)]
mod utils;

pub fn run() -> Result<(), Box<dyn Error>> {
    let context = Context::new();

    let _ = context_handler::handle_context(context)?;

    Ok(())
}
