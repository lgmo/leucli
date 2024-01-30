use std::io;
use std::process::Command;
use crate::context::Context;
use crate::git_manager;

// fn validate_context(context: &Context) -> Result<(), &'static str> {
//     if git_manager::is_git_project(&context.cwd).unwrap() {
//         return Ok(());
//     }
//
//     Err("O diretório atual não é um projeto git.")
// }
//
//
// pub fn handle_pull(context: Context) -> Result<(), &'static str> {
//     validate_context(&context)?;
//     Command::new("git").args(vec!["pull"]).status().unwrap();
//     Ok(())
// }
//
// pub fn handle_wip(context: Context) -> Result<(), &'static str> {
//     validate_context(&context)?;
//     Command::new("git").args(vec!["add", "."]).status().unwrap();
//     Command::new("git").args(vec!["commit", "-m", "\"wip\""])
//         .status().unwrap();
//     Ok(())
// }
//
// pub fn handle_ucommit(context: Context) -> Result<(), &'static str> {
//     validate_context(&context)?;
//     Command::new("git").args(vec!["reset", "HEAD^"])
//         .status().unwrap();
//     Ok(())
// }
//
// pub fn handle_commit(context: Context) -> Result<(), &'static str> {
//     validate_context(&context)?;
//
//     let vargs = if context.args.len() < 3 {
//         vec!["commit"]
//     } else {
//         vec!["commit", "-m", &context.args[2]]
//     };
//
//
//     Command::new("git").args(vec!["add", "."]).status().unwrap();
//     Command::new("git")
//         .args(vargs).status().unwrap();
//
//     Ok(())
// }
//
// pub fn handle_push(context: Context) -> Result<(), &'static str> {
//     validate_context(&context)?;
//     let branch = git_manager::get_current_branch(&context.cwd).unwrap();
//     println!("Push da branch {} para o github. Continuar? [s/n]", branch);
//
//     let mut ans: String = String::new();
//
//     loop {
//         let _ = io::stdin().read_line(&mut ans).unwrap();
//         ans = ans.trim().to_lowercase();
//
//         if ans == "s" || ans == "sim" || ans == "n" || ans == "nao" || ans == "não" {
//             break;
//         }
//         println!("Opção inválida! Tente novamente: ");
//     }
//
//     if ans == "s" || ans == "sim" {
//         Command::new("git")
//             .args(vec!["push", "-u", "origin", branch.as_str()]).status().unwrap();
//     }
//
//     Ok(())
// }
//
// pub fn handle_gremlin(context: Context) -> Result<(), &'static str> {
//     validate_context(&context)?;
//     Command::new("sudo").args(vec!["docker", "compose", "up", "-d", "graph-db"])
//         .status().unwrap();
//     Ok(())
// }
//
// pub fn handle_down(context: Context) -> Result<(), &'static str> {
//     validate_context(&context)?;
//     Command::new("sudo").args(vec!["docker", "compose", "down"])
//         .status().unwrap();
//     Ok(())
// }
//
// pub fn handle_rbranch(context: Context) -> Result<(), &'static str> {
//     validate_context(&context)?;
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
