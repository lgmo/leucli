use std::error::Error;
use std::process::Command;
use crate::command::CommandKind::{Nil, PULL};
use crate::settings::Settings;

enum CommandKind {
    PULL,
    Nil,
}

pub fn get_strategy(settings: Settings) -> fn() -> Result<(), Box<dyn Error>> {
    let result  = match get_kind(settings.args) {
        PULL => {
            || -> Result<(), Box<dyn Error>> {
                Command::new("git").args(vec!["pull"]).status()?;
                Ok(())
            }
        },
        _ => {
            || -> Result<(), Box<dyn Error>> { Ok(()) }
        },
    };

    result
}

fn get_kind(args: Vec<String>) -> CommandKind {
    if args.len() < 2 {
        return Nil;
    }
    else if &args[1] == "pull" {
        return PULL;
    }

    Nil
}