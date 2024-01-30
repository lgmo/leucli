use std::env;
use std::path::PathBuf;
use crate::file_manager;
use crate::file_manager::{FileManager, LinuxFileManager};

pub struct Context {
    pub cwd: PathBuf,
    pub args: Vec<String>,
}

impl Context {
    pub fn new(file_manager: Box<dyn FileManager>) -> Context {
        let cwd = file_manager.get_cwd();
        let args = env::args().collect();

        Context { cwd, args }
    }
}
