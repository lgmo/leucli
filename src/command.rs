use std::error::Error;
use std::io;
use std::process::Command;
use crate::context::Context;

pub trait CommandStrategy {
    fn execute (&self) -> Result<(), Box<dyn Error>>;
}

pub fn build_executor(settings: Context) -> Result<Box<dyn CommandStrategy>, &'static str> {
    let result: Box<dyn CommandStrategy>;
    if settings.args.len() < 2 {
        result = Box::new(NilStrategy {});
    } else if &settings.args[1] == "pull" {
        result = Box::new(PullStrategy {});
    } else if &settings.args[1] == "commit" {
        if settings.args.len() < 3 {
            result = Box::new(CommitStrategy { message: None });
        } else {
            result = Box::new(CommitStrategy {
                message: Some(settings.args[2].to_owned())
            });
        }
    } else if &settings.args[1] == "push" {
        result = Box::new(PushStrategy { branch: settings.project.unwrap().current_branch })
    } else if &settings.args[1] == "status" {
        result = Box::new(StatusStrategy {});
    } else if &settings.args[1] == "gremlin" {
        result = Box::new(GremlinStrategy {})
    } else {
        result =  Box::new(NilStrategy {});
    }

    Ok(result)
}

struct NilStrategy {}

impl CommandStrategy for NilStrategy {
    fn execute(&self) -> Result<(), Box<dyn Error>> { Ok(()) }
}

struct PullStrategy {}

impl CommandStrategy for PullStrategy {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        Command::new("git").args(vec!["pull"]).status().unwrap();
        Ok(())
    }
}

struct CommitStrategy {
    message: Option<String>,
}

impl CommandStrategy for CommitStrategy {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let vargs = match &self.message {
            Some(msg) => { vec!["commit", "-m", msg.as_str()] },
            _ => { vec!["commit"] },
        };


        Command::new("git").args(vec!["add", "."]).status()?;
        Command::new("git")
            .args(vargs).status()?;

        Ok(())
    }
}

struct PushStrategy {
    branch: String,
}

impl CommandStrategy for PushStrategy {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        println!("Push da branch {} para o github. Continuar? [s/n]", &self.branch);

        let mut ans: String = String::new();

        loop {
            let _ = io::stdin().read_line(&mut ans)?;
            ans = ans.trim().to_lowercase();

            if ans == "s" || ans == "sim" || ans == "n" || ans == "nao" || ans == "não" {
                break;
            }
            println!("Opção inválida! Tente novamente: ");
        }

        if ans == "s" || ans == "sim" {
            Command::new("git")
                .args(vec!["push", "-u", "origin", &self.branch.as_str()]).status()?;
        }

        Ok(())
    }
}

struct StatusStrategy {}

impl CommandStrategy for StatusStrategy {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        Command::new("git").args(vec!["status"]).status().unwrap();
        Ok(())
    }
}

struct GremlinStrategy {}

impl CommandStrategy for GremlinStrategy {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        Command::new("sudo").args(vec!["docker", "compose", "up", "-d", "graph-db"])
            .status().unwrap();
        Ok(())
    }
}

