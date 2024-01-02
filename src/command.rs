use std::error::Error;
use std::process::Command;
use crate::settings::Settings;

pub trait Strategy {
    fn execute (&self) -> Result<(), Box<dyn Error>>;
}

pub fn get_strategy(settings: Settings) -> Result<Box<dyn Strategy>, &'static str> {
    let result: Box<dyn Strategy>;
    if settings.args.len() < 2 {
        result =  Box::new(NilStrategy {});
    }
    else if &settings.args[1] == "pull" {
        result = Box::new(PullStrategy {});
    } else if &settings.args[1] == "commit" {
        if settings.args.len() < 3 {
            result = Box::new(CommitStrategy { message: None });
        } else {
            result = Box::new(CommitStrategy {
                message: Some(settings.args[2].to_owned())
            });
        }
    } else {
        result =  Box::new(NilStrategy {});
    }

    Ok(result)
}

struct NilStrategy {}

impl Strategy for NilStrategy {
    fn execute(&self) -> Result<(), Box<dyn Error>> { Ok(()) }
}

struct PullStrategy {}

impl Strategy for PullStrategy {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        Command::new("git").args(vec!["pull"]).status().unwrap();
        Ok(())
    }
}

struct CommitStrategy {
    message: Option<String>,
}

impl Strategy for CommitStrategy {
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




