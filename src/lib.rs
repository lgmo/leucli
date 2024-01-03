use std::env;
use std::error::Error;
use crate::git_project::GitProject;
use crate::context::Context;

mod file_manager;
mod git_project;
mod git_manager;
mod context;
mod command;
#[cfg(test)]
mod utils;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cwd = file_manager::get_cwd();
    let project = GitProject::build(&cwd)?;
    let args = env::args().collect();
    let settings = Context::build(cwd, project, args)?;


    let executor = command::build_executor(settings)?;
    let _ = executor.execute();

    Ok(())
}
