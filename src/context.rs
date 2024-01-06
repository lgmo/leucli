use std::env;
use std::path::PathBuf;
use crate::file_manager;

pub struct Context {
    pub cwd: PathBuf,
    pub args: Vec<String>,
}

impl Context {
    pub fn new<'a>() -> Context {
        let cwd = file_manager::get_cwd();
        let args = env::args().collect();

        Context { cwd, args }
    }
}
