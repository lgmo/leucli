mod git_handler;

use std::collections::HashMap;
use std::error::Error;
use std::process;
use crate::command::{Command, CommandInput};
use crate::git_manager::{GitManager, LinuxGitManager};
use crate::scopes::Scope;

fn check_scope(scope: &Scope) -> Result<(), String> {
    if scope.name == "default" {
        return Ok(());
    } else if scope.name == "git" {
        if LinuxGitManager.cwd_is_git_project() {
            return Ok(());
        } else {
            return Err("Not a git project".to_string());
        }
    } else {
        return Ok(());
    }
}

pub fn handle(
    command_input: CommandInput,
    scope_table: HashMap<String, Scope>,
    command_table: HashMap<String, Command>,
) -> Result<(), Box<dyn Error>> {
    let command_name = command_table.get(&command_input.name).unwrap().name.clone();
    let scope = scope_table.get(&command_name).unwrap();
    let _ = check_scope(&scope)?;
    if scope.name == "git" {
        git_handler::handle_git_command(command_input)?;
    } else {
        let command = command_table.get(&command_input.name).unwrap();
        if command.execution_list.len() == 0 {
            return Err("No execution list".to_string().into());
        } else if command.execution_list.len() == 1 {
            let mut args = Vec::from(vec!["-c", &command.execution_list[0]]);
            args.extend(command_input.args);
            process::Command::new("sh").args(args)
                .status().unwrap();
        } else {
            for execution in &command.execution_list {
                process::Command::new("sh").args(vec!["-c", execution])
                    .status().unwrap();
            }
        }
    }

    Ok(())
}