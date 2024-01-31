use std::error::Error;
use std::path::PathBuf;
use serde_json::Value;
use crate::file_manager::{FileManager, LinuxFileManager};


#[derive(Debug)]
pub struct Config {
    pub scopes: Option<Value>,
    pub commands: Option<Value>,
}

fn parse_to_json(config: String) -> Result<Value, String> {
    let json;
    match serde_json::from_str(config.as_str()) {
        Ok(v) => { json = v; },
        _ => { return Err(String::from("Bad json syntax in .leuclirc")); }
    }

    Ok(json)
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        let mut path = PathBuf::from(simple_home_dir::home_dir().unwrap());
        path.push(".leuclirc");

        let config;
        match LinuxFileManager.get_file_content(path.as_path()) {
            Ok(v) => { config = v; },
            _ => { return Ok(Config { scopes: None, commands: None }); }
        }

        let json = parse_to_json(config)?;

        let scopes = json.get("scopes").map(|s| s.to_owned());
        let commands = json.get("commands").map(|s| s.to_owned());

        Ok(Config { scopes, commands })
    }
}