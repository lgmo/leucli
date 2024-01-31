mod git_handler;

use std::collections::HashMap;
use std::error::Error;
use crate::command::CommandInput;
use crate::git_manager::{GitManager, LinuxGitManager};
use crate::scopes::Scope;

pub fn handle(
    command_input: CommandInput,
    scope_table: HashMap<String, Scope>
) -> Result<(), Box<dyn Error>> {
    if scope_table.get(&command_input.name).unwrap().name == "git" {
        if ! LinuxGitManager.cwd_is_git_project() {
            return Err(
                format!("Cannot use command '{}', of scope 'git' in a non git project directory", command_input.name)
                    .into()
            );
        } else {
            git_handler::handle_git_command(command_input)?;
        }
    } else {
        println!("Scope not found");
    }

    Ok(())
}