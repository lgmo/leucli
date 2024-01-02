use std::path::PathBuf;
use crate::file_manager;

pub fn get_test_dirs_path() -> PathBuf{
    let mut path = file_manager::get_cwd();
    let _ = &path.push("tests/test_dirs");
    path
}

