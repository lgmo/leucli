use std::error::Error;
use std::io;
use std::process::Command;
use crate::command::CommandInput;
use crate::git_manager::{GitManager, LinuxGitManager};

pub fn handle_git_command(command_input: CommandInput) -> Result<(), Box<dyn Error>>{
    if command_input.name == "pull" {
        handle_pull(command_input)?;
    } else if command_input.name == "wip" {
        handle_wip(command_input)?;
    } else if command_input.name == "undo-commit" {
        handle_undo_commit(command_input)?;
    } else if command_input.name == "commit" {
        handle_commit(command_input)?;
    } else if command_input.name == "push" {
        handle_push(command_input)?;
    }
    // else if command_input.name == "rbranch" {
    //     handle_rbranch(command_input);
    // }

    Ok(())
}

pub fn handle_pull(command_input: CommandInput) -> Result<(), &'static str> {
    let mut args = Vec::from(vec!["pull"]);
    args.extend_from_slice(command_input.args.iter().map(|s| s.as_str())
        .collect::<Vec<&str>>().as_slice());
    Command::new("git").args(args).status().unwrap();
    Ok(())
}

pub fn handle_wip(command_input: CommandInput) -> Result<(), &'static str> {
    Command::new("git").args(vec!["add", "."]).status().unwrap();

    let mut args = Vec::from(vec!["commit", "-m", "\"wip\""]);
    args.extend_from_slice(command_input.args.iter().map(|s| s.as_str())
        .collect::<Vec<&str>>().as_slice());

    Command::new("git").args(args)
        .status().unwrap();
    Ok(())
}

pub fn handle_undo_commit(command_input: CommandInput) -> Result<(), &'static str> {
    let mut args = Vec::from(vec!["reset", "head^"]);
    args.extend_from_slice(command_input.args.iter().map(|s| s.as_str())
        .collect::<Vec<&str>>().as_slice());

    Command::new("git").args(args)
        .status().unwrap();
    Ok(())
}

pub fn handle_commit(command_input: CommandInput) -> Result<(), &'static str> {
    let mut args = Vec::from(vec!["commit", "-m"]);
    args.extend_from_slice(command_input.args.iter().map(|s| s.as_str())
        .collect::<Vec<&str>>().as_slice());


    Command::new("git").args(vec!["add", "."]).status().unwrap();
    Command::new("git")
        .args(args).status().unwrap();

    Ok(())
}

pub fn handle_push(command_input: CommandInput) -> Result<(), &'static str> {
    let branch = LinuxGitManager.get_current_branch().unwrap();
    println!("Push da branch {} para o github. Continuar? [s/n]", branch);

    let mut ans: String = String::new();

    loop {
        let _ = io::stdin().read_line(&mut ans).unwrap();
        ans = ans.trim().to_lowercase();

        if ans == "s" || ans == "sim" || ans == "n" || ans == "nao" || ans == "não" {
            break;
        }
        println!("Opção inválida! Tente novamente: ");
    }

    let mut args = Vec::from(vec!["push", "-u", "origin", branch.as_str()]);
    args.extend_from_slice(command_input.args.iter().map(|s| s.as_str())
        .collect::<Vec<&str>>().as_slice());

    if ans == "s" || ans == "sim" {
        Command::new("git").args(args).status().unwrap();
    }

    Ok(())
}

// pub fn handle_rbranch(command_input: CommandInput) -> Result<(), &'static str> {
//     let vargs = if context.args.len() < 3 {
//         vec!["branch"]
//     } else {
//         vec!["branch", "-m", &context.args[2]]
//     };
//
//
//     Command::new("git").args(vargs)
//         .status().unwrap();
//
//     Ok(())
// }
